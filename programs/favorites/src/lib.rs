use anchor_lang::prelude::*;

declare_id!("FXGJqSQMfdKpZhRUwuSE7WQPhk3wdFEfEcsjqZGJW8oj");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        let user_public_key: Pubkey = context.accounts.user.key();
        msg!("Greetings from: {:?}", context.program_id);
        msg!(
            "User public key: {user_public_key}'s favorite number is {number}, favorite color is {color}",
        );

        msg!(
            "User's hobbies are: {:?}",
            hobbies
        );

        context.accounts.favorite.set_inner(Favorites {
                number,
                color,
                hobbies
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds=[b"favorites", user.key().as_ref()],
    bump)]
    pub favorite: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,
    
    #[max_len(5, 50)]
    pub hobbies: Vec<String>
}
