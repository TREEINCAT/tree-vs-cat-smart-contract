use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

declare_id!("FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF");

const TIC: u64 = 1_000_000_000; // 10^9 decimals
const BURN_PCT: u128 = 95;

#[program]
pub mod tic_world {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;

        require_keys_eq!(
            ctx.accounts.treasury_token_account.owner,
            ctx.accounts.authority.key(),
            ErrorCode::InvalidTreasuryOwner
        );
        require_keys_eq!(
            ctx.accounts.treasury_token_account.mint,
            ctx.accounts.tic_mint.key(),
            ErrorCode::InvalidMint
        );

        game_state.authority = ctx.accounts.authority.key();
        game_state.tic_mint = ctx.accounts.tic_mint.key();
        game_state.treasury_token_account = ctx.accounts.treasury_token_account.key();
        game_state.bump = ctx.bumps.game_state;

        Ok(())
    }

    pub fn buy_fish(ctx: Context<BuyItem>, kind: String) -> Result<()> {
        let cost = match kind.to_ascii_lowercase().as_str() {
            "small" => 500 * TIC,
            "medium" => 750 * TIC,
            "large" => 1000 * TIC,
            _ => return err!(ErrorCode::InvalidKind),
        };

        process_purchase(&ctx, cost)?;

        emit!(FishPurchased {
            buyer: ctx.accounts.player.key(),
            kind,
            amount_paid: cost
        });
        Ok(())
    }

    pub fn buy_cat(ctx: Context<BuyItem>, cat_type: String) -> Result<()> {
        let cost = match cat_type.as_str() {
            "Stray" => 250 * TIC,
            "Kitten" => 400 * TIC,
            "Tabby" => 550 * TIC,
            "Persian" => 700 * TIC,
            "MaineCoon" => 850 * TIC,
            "Siamese" => 1000 * TIC,
            "Bengal" => 2000 * TIC,
            "ShadowCat" => 3000 * TIC,
            _ => return err!(ErrorCode::InvalidKind),
        };

        process_purchase(&ctx, cost)?;

        emit!(CatPurchased {
            buyer: ctx.accounts.player.key(),
            cat_type,
            amount_paid: cost
        });
        Ok(())
    }

    pub fn buy_tree(ctx: Context<BuyItem>, tree_type: String) -> Result<()> {
        let cost = match tree_type.as_str() {
            "Seedling" => 250 * TIC,
            "Sapling" => 400 * TIC,
            "Oak" => 550 * TIC,
            "Pine" => 700 * TIC,
            "Redwood" => 850 * TIC,
            "Willow" => 1000 * TIC,
            "Baobab" => 1150 * TIC,
            "Palm" => 1300 * TIC,
            "Bonsai" => 1450 * TIC,
            "Maple" => 1600 * TIC,
            "Cactus" => 2150 * TIC,
            "WorldTree" => 4300 * TIC,
            _ => return err!(ErrorCode::InvalidKind),
        };

        process_purchase(&ctx, cost)?;

        emit!(TreePurchased {
            buyer: ctx.accounts.player.key(),
            tree_type,
            amount_paid: cost
        });
        Ok(())
    }
}

#[inline(never)]
fn process_purchase(ctx: &Context<BuyItem>, cost: u64) -> Result<()> {
    let game_state = &ctx.accounts.game_state;

    // Validate accounts
    require_keys_eq!(game_state.tic_mint, ctx.accounts.tic_mint.key(),);
    require_keys_eq!(
        ctx.accounts.player_token_account.mint,
        ctx.accounts.tic_mint.key(),
        ErrorCode::InvalidMint
    );
    require_keys_eq!(
        ctx.accounts.treasury_token_account.key(),
        game_state.treasury_token_account,
        ErrorCode::InvalidTreasuryAccount
    );
    require_keys_eq!(
        ctx.accounts.treasury_token_account.mint,
        ctx.accounts.tic_mint.key(),
    );
    require_keys_eq!(
        ctx.accounts.player_token_account.owner,
        ctx.accounts.player.key(),
        ErrorCode::InvalidTokenOwner
    );
    require!(
        ctx.accounts.player_token_account.amount >= cost,
        ErrorCode::InsufficientFunds
    );

    // Burn 95%
    let burn_amount = ((cost as u128 * BURN_PCT) / 100) as u64;
    let cpi_accounts = Burn {
        mint: ctx.accounts.tic_mint.to_account_info(),
        from: ctx.accounts.player_token_account.to_account_info(),
        authority: ctx.accounts.player.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::burn(cpi_ctx, burn_amount)?;

    // Transfer 5% to treasury
    let treasury_amount = cost - burn_amount;
    let cpi_accounts = Transfer {
        from: ctx.accounts.player_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.player.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, treasury_amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + GameState::INIT_SPACE,
        seeds = [b"game_state_v2"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    pub tic_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyItem<'info> {
    #[account(mut, seeds = [b"game_state_v2"], bump = game_state.bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub tic_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct FishPurchased {
    pub buyer: Pubkey,
    pub kind: String,
    pub amount_paid: u64,
}

#[event]
pub struct CatPurchased {
    pub buyer: Pubkey,
    pub cat_type: String,
    pub amount_paid: u64,
}

#[event]
pub struct TreePurchased {
    pub buyer: Pubkey,
    pub tree_type: String,
    pub amount_paid: u64,
}

#[account]
#[derive(InitSpace)]
pub struct GameState {
    pub authority: Pubkey,
    pub tic_mint: Pubkey,
    pub treasury_token_account: Pubkey,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    InvalidMint,
    InvalidTreasuryAccount,
    InsufficientFunds,
    Unauthorized,
    InvalidTokenOwner,
    InvalidTreasuryOwner,
    InvalidKind,
}
