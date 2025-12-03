//-------------------------------------------------------------------------------
///
/// TASK: Implement the add comment functionality for the Twitter program
///
/// Requirements:
/// - Validate that comment content doesn't exceed maximum length
/// - Initialize a new comment account with proper PDA seeds
/// - Set comment fields: content, author, parent tweet, and bump
/// - Use content hash in PDA seeds for unique comment identification
///
///-------------------------------------------------------------------------------
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    // TODO: Implement add comment functionality
    let comment_acc = &mut ctx.accounts.comment;
    if comment_content.as_bytes().len() > COMMENT_LENGTH {
        return err!(TwitterError::CommentTooLong);
    }
    comment_acc.comment_author = ctx.accounts.comment_author.key();
    comment_acc.parent_tweet = ctx.accounts.tweet.key();
    comment_acc.content = comment_content;
    comment_acc.bump = ctx.bumps.comment;

    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub comment_author: Signer<'info>,

    #[account(
        init,
        payer = comment_author,
        space = 8 + Tweet::INIT_SPACE,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            {hash(comment_content.as_bytes()).to_bytes().as_ref()},
            tweet.key().as_ref()
        ],
        bump,
    )]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    pub system_program: Program<'info, System>,
}