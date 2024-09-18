use anchor_lang::prelude::*;

mod constants;
mod state;
mod Contexts;
use Contexts::*;
mod error;
mod helpers;


declare_id!("Af1KM8dxY7Uawhr6YG68vw6fpomTzom7zDUWap2J7p2K");

#[program]
pub mod stormbreaker {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,       //Fee as basis points
        authority: Option<Pubkey>, //update auth (if required)
        ) -> Result<()> {
            // Initialize our AMM config
        ctx.accounts.init(&ctx.bumps, seed, fee, authority)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64, //Amount of LP token to claim
        max_x: u64, //Max amount of X we are willing to deposit
        max_y: u64, //Max amount of Y we are willing to deposit
        expiration: u64,
    ) -> Result<()> {
        // Deposit Liquidity to swap
        ctx.accounts.deposit(amount, max_x, max_y, expiration)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64, //Amount of liquidity tokens to burn
        min_x: u64, //Minimum amount of liquidity we are willing to recieve
        min_y: u64, //Minimun amount of liquidity we are willing recieve
        expiration: u64,
    ) -> Result<()> {
        //Withdraw liquidity from
        ctx.accounts.withdraw(amount, min_x, min_y, expiration)
    }

    pub fn swap(
        ctx: Context<Swap>,
        is_x: bool,
        amount: u64, // Amount of tokens we deposit
        min: u64, // Minimum amount of token I'd be willing to withdraw
        expiration: u64,
    ) -> Result<()> {
        // Swap Token X for Token Y or vice versa
        ctx.accounts.swap(is_x, amount, min, expiration)
    }

    pub fn lock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.lock()
    }

    pub fn unclock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.unlock()
    }
}
