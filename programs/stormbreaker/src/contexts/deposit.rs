use anchor_lang::{accounts::account, prelude::*, solana_program::address_lookup_table::instruction};
use constant_product_curve::ConstantProduct;
use crate::state::config::Config;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, mint_to, MintTo},
};
use crate::error::AmmError;
use crate::{assert_non_zero,assert_not_expired,assert_not_locked};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"lp",
            config.key().as_ref()
        ],
        bump = config.lp_bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )] 
    pub user_vault_y: InterfaceAccount<'info, TokenAccount>,
    #[acount(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_vault_lp: InterfaceAccount<'info, TokenAccount>,
    ///CHECK: pda used just per signing purpose
    #[account(seeds = 
        [b"auth"],
        bump = config.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [
            b"config",
            config.seeds.to_le_bytes().as_ref()
        ],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>   
}

impl<'info> Deposit<'info> {
    pub fn deposit(
        &mut self,
        amount: u64, //amount of LP token the depositor wants to claim
        max_x: u64, //maximum amount of token X that the depositor is willing to deposit
        max_y: u64, //maximum amount of token Y the depositor wants to deposit
        expiration: i64, // the time limit offer
    )   -> Result<()> {
        assert_not_locked!(self.config.locked);
        assert_not_expired!(expiration);
        assert_non_zero!([amount, max_x, max_y]);

        let (x, y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 {
            true => (max_x, max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6).map_err(AmmError::from)?;
                    (amounts.x, amounts.y)
            }
        };

        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);
    }

}