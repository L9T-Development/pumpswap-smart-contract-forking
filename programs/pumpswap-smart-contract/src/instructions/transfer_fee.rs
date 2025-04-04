use std::ops::{Div, Mul};

use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use spl_token::instruction::sync_native;

use crate::{
    constants::{BONDING_CURVE, CONFIG, GLOBAL},
    errors::PumpfunError,
    state::{BondingCurve, Config},
    utils::{
        convert_from_float, convert_to_float, sol_transfer_with_signer, token_transfer_with_signer,
    },
};

#[derive(Accounts)]
pub struct TransferFee<'info> {
    /// CHECK: Safe
    #[account(
        mut,
        constraint = global_config.team_wallet == *team_wallet.key @PumpfunError::IncorrectAuthority
    )]
    team_wallet: UncheckedAccount<'info>,

    #[account(
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    #[account(
        mut,
        seeds = [BONDING_CURVE.as_bytes(), &coin_mint.key().to_bytes()],
        bump
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    /// CHECK
    #[account(
        mut,
        seeds = [GLOBAL.as_bytes()],
        bump,
    )]
    global_vault: UncheckedAccount<'info>,

    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,

    coin_mint: Box<Account<'info, Mint>>,

    #[account(
        address = spl_token::native_mint::ID
    )]
    pc_mint: Box<Account<'info, Mint>>,

    /// CHECK: Safe. The user wallet create the pool
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: verified in transfer instruction
    #[account(
        mut,
        seeds = [
            global_vault.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            coin_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    global_token_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            team_wallet.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            coin_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    team_ata: UncheckedAccount<'info>,

    /// CHECK: Safe. wsol account of global_vault
    #[account(
        mut,
        associated_token::mint = pc_mint,
        associated_token::authority = global_vault
    )]
    global_wsol_account: Box<Account<'info, TokenAccount>>,
}

impl<'info> TransferFee<'info> {
    pub fn process(&mut self, global_vault_bump: u8) -> Result<()> {
        let global_config = &mut self.global_config;
        let bonding_curve = &mut self.bonding_curve;

        //  check curve is completed
        require!(
            bonding_curve.is_completed == true,
            PumpfunError::CurveNotCompleted
        );

        let lamport_on_curve = bonding_curve.reserve_lamport - bonding_curve.init_lamport;

        let fee_in_float = convert_to_float(lamport_on_curve, self.coin_mint.decimals)
            .div(100_f64)
            .mul(global_config.platform_migration_fee);

        let fee_lamport = convert_from_float(fee_in_float, self.coin_mint.decimals);

        //  updated this 1 as 0.4
        //  1 + 0.01715 - pool create fee

        //  0.3 - market create fee
        let init_pc_amount = lamport_on_curve - fee_lamport - 1_400_000_000;

        let coin_amount = (init_pc_amount as u128 * bonding_curve.reserve_token as u128
            / bonding_curve.reserve_lamport as u128) as u64;
        let fee_token = bonding_curve.reserve_token - coin_amount;

        msg!(
            "Raydium Input:: Token: {:?}  Sol: {:?}",
            coin_amount,
            init_pc_amount
        );
        msg!("Fee percent: {:?}", global_config.platform_migration_fee);
        msg!("Fee:: Token: {:?}  Sol: {:?}", fee_token, fee_lamport);

        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL.as_bytes(), &[global_vault_bump]]];

        //  transfer 0.3 SOL to signer for market creation fee
        sol_transfer_with_signer(
            self.global_vault.to_account_info(),
            self.payer.to_account_info(),
            &self.system_program,
            signer_seeds,
            300_000_000,
        )?;

        //  transfer migration fee to team wallet
        sol_transfer_with_signer(
            self.global_vault.to_account_info(),
            self.team_wallet.to_account_info(),
            &self.system_program,
            signer_seeds,
            fee_lamport,
        )?;
        token_transfer_with_signer(
            self.global_token_account.to_account_info(),
            self.global_vault.to_account_info(),
            self.team_ata.to_account_info(),
            &self.token_program,
            signer_seeds,
            fee_token,
        )?;

        //  sync WSOL account of global_acocunt
        sol_transfer_with_signer(
            self.global_vault.to_account_info(),
            self.global_wsol_account.to_account_info(),
            &self.system_program,
            signer_seeds,
            init_pc_amount,
        )?;

        let sync_native_ix = sync_native(&spl_token::id(), &self.global_wsol_account.key())?;
        invoke_signed(
            &sync_native_ix,
            &[self.global_wsol_account.to_account_info().clone()],
            signer_seeds,
        )?;

        Ok(())
    }
}
