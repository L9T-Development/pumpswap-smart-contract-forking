use crate::constants::LAMPORT_DECIMALS;
use crate::errors::*;
use crate::events::CompleteEvent;
use crate::utils::*;
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use core::fmt::Debug;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[account]
pub struct Config {
    pub authority: Pubkey,
    //  use this for 2 step ownership transfer
    pub pending_authority: Pubkey,

    pub team_wallet: Pubkey,

    pub init_bonding_curve: f64, // bonding curve init percentage. The remaining amount is sent to team wallet for distribution to agent

    pub platform_buy_fee: f64, //  platform fee percentage
    pub platform_sell_fee: f64,
    pub platform_migration_fee: f64,

    pub curve_limit: u64, //  lamports to complete te bonding curve

    pub lamport_amount_config: AmountConfig<u64>,
    pub token_supply_config: AmountConfig<u64>,
    pub token_decimals_config: AmountConfig<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum AmountConfig<T: PartialEq + PartialOrd + Debug> {
    Range { min: Option<T>, max: Option<T> },
    Enum(Vec<T>),
}

impl<T: PartialEq + PartialOrd + Debug> AmountConfig<T> {
    pub fn validate(&self, value: &T) -> Result<()> {
        match self {
            Self::Range { min, max } => {
                if let Some(min) = min {
                    if value < min {
                        msg!("value {value:?} too small, expected at least {min:?}");
                        return Err(ValueTooSmall.into());
                    }
                }
                if let Some(max) = max {
                    if value > max {
                        msg!("value {value:?} too large, expected at most {max:?}");
                        return Err(ValueTooLarge.into());
                    }
                }

                Ok(())
            }
            Self::Enum(options) => {
                if options.contains(value) {
                    Ok(())
                } else {
                    msg!("invalid value {value:?}, expected one of: {options:?}");
                    Err(ValueInvalid.into())
                }
            }
        }
    }
}

#[account]
pub struct BondingCurve {
    pub token_mint: Pubkey,
    pub creator: Pubkey,

    pub init_lamport: u64,

    pub reserve_lamport: u64,
    pub reserve_token: u64,

    pub is_completed: bool,
}
pub trait BondingCurveAccount<'info> {
    // Updates the token reserves in the liquidity pool
    fn update_reserves(
        &mut self,
        global_config: &Account<'info, Config>,
        reserve_one: u64,
        reserve_two: u64,
    ) -> Result<bool>;

    fn swap(
        &mut self,
        global_config: &Account<'info, Config>,
        token_mint: &Account<'info, Mint>,
        global_ata: &mut AccountInfo<'info>,
        user_ata: &mut AccountInfo<'info>,
        source: &mut AccountInfo<'info>,
        team_wallet: &mut AccountInfo<'info>,
        team_wallet_ata: &mut AccountInfo<'info>,
        pool_creator: &mut AccountInfo<'info>,
        amount: u64,
        direction: u8,
        minimum_receive_amount: u64,

        user: &Signer<'info>,
        signer: &[&[&[u8]]],

        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<u64>;

    fn simulate_swap(
        &self,
        global_config: &Account<'info, Config>,
        token_mint: &Account<'info, Mint>,
        amount: u64,
        direction: u8,
    ) -> Result<u64>;

    fn cal_amount_out(
        &self,
        amount: u64,
        token_one_decimals: u8,
        direction: u8,
        platform_sell_fee: f64,
        platform_buy_fee: f64,
    ) -> Result<(u64, u64)>;
}

impl<'info> BondingCurveAccount<'info> for Account<'info, BondingCurve> {
    *** private ***
}
