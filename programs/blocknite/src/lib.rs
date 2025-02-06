use anchor_lang::prelude::*;
use orao_solana_vrf::cpi::{accounts::RequestV2, request_v2};
use orao_solana_vrf::{
    program::OraoVrf, state::NetworkState, CONFIG_ACCOUNT_SEED, RANDOMNESS_ACCOUNT_SEED,
};
declare_id!("2tYcASh4cCNAnxAygN3f6cEze6udchSb6jPc5xP9RdAE");

#[program]
pub mod blocknite {
    use std::u64;

    use orao_solana_vrf::state::RandomnessAccountData;

    use super::*;

    pub fn create_battle(
        ctx: Context<CreateBattle>,
        id: [u8; 32],
        win_chance_bp: u64,
    ) -> Result<()> {
        if id == [0_u8; 32] {
            return Err(CustomError::InvalidForce.into());
        }
        let cpi_program = ctx.accounts.vrf.to_account_info();
        let cpi_accounts = RequestV2 {
            payer: ctx.accounts.signer.to_account_info(),
            network_state: ctx.accounts.config.to_account_info(),
            treasury: ctx.accounts.treasury.to_account_info(),
            request: ctx.accounts.randomness.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        request_v2(cpi_ctx, id)?;
        ctx.accounts.battle.id = id;
        ctx.accounts.battle.win_chance_bp = win_chance_bp;
        Ok(())
    }
    pub fn end_battle(ctx: Context<EndBattle>, id: [u8; 32]) -> Result<()> {
        if ctx.accounts.randomness.data_is_empty() {
            return Err(CustomError::UninitializedAccount.into());
        }
        let account = RandomnessAccountData::try_deserialize(
            &mut &ctx.accounts.randomness.data.borrow()[..],
        )?;
        let randomness = account.fulfilled_randomness();
        match randomness {
            Some(rand) => {
                let num = u64::from_le_bytes(rand[..8].try_into().unwrap());
                let threshold = u64::MAX / 10000 * ctx.accounts.battle.win_chance_bp;
                if num < threshold {
                    // win
                } else {
                    // loss
                }
            }
            None => return Err(CustomError::RandomnessNotFound.into()),
        };
        Ok(())
    }
}
#[error_code]
pub enum CustomError {
    #[msg("Invalid force")]
    InvalidForce,
    #[msg("Uninitialized Account")]
    UninitializedAccount,
    #[msg("Randomness not found")]
    RandomnessNotFound,
}
#[account]
pub struct Battle {
    pub id: [u8; 32],
    pub win_chance_bp: u64,
}
#[derive(Accounts)]
#[instruction(id: [u8; 32])]
pub struct CreateBattle<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [RANDOMNESS_ACCOUNT_SEED, &id],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub randomness: AccountInfo<'info>,
    #[account(
        init,
        seeds = [b"battle", signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + 32 + 8,
    )]
    pub battle: Account<'info, Battle>,
    /// VRF treasury account, it'll be the `treasury` account in the CPI call.
    /// CHECK:
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    /// CHECK: VRF on-chain state account, it'll be the `network_state` account in the CPI call.
    pub config: Account<'info, NetworkState>,
    /// VRF program address to invoke CPI
    pub vrf: Program<'info, OraoVrf>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(id: [u8; 32])]
pub struct EndBattle<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [RANDOMNESS_ACCOUNT_SEED, &id],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub randomness: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"battle", signer.key().as_ref()],
        bump,
        close = signer
    )]
    pub battle: Account<'info, Battle>,
    pub system_program: Program<'info, System>,
}
