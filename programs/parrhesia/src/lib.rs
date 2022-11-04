use anchor_lang::prelude::*;

pub mod states;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("DJR4rsWxMy6SDrqgVx2BwnGHWkSRqFGXYEG2sSRrrDKV");

#[program]
mod parrhesia {
    use super::*;

    pub fn create_profile(
        ctx: Context<CreateProfile>,
        name: String,
        bio: String
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.signer.key();
        profile.name = name;
        profile.bio = bio;

        Ok(())
    }

    
    pub fn create_membership_plan(
        ctx: Context<CreateMembershipPlan>,
        name: String,
        description: String,
        amount: u64
    ) -> Result<()> {
        let membership_plan = &mut ctx.accounts.membership_plan;
        membership_plan.authority = ctx.accounts.signer.key();
        membership_plan.name = name;
        membership_plan.description = description;
        membership_plan.amount = amount;
        membership_plan.count = 0;

        Ok(())
    }

    pub fn buy_membership(ctx: Context<BuyMembership>) -> Result<()> {
        let transaction_msg = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.signer.key(),
            &ctx.accounts.membership_plan.key(),
            ctx.accounts.membership_plan.amount
        );
        anchor_lang::solana_program::program::invoke(
            &transaction_msg,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.membership_plan.to_account_info(),
            ]
        );
        let membership_plan = &mut ctx.accounts.membership_plan;
        membership_plan.count += 1;
        Ok(())
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        body: String
    ) -> Result<()> {
        
        let post = &mut ctx.accounts.post;
        post.authority = ctx.accounts.signer.key();
        post.body = body;

        Ok(())
    }


    pub fn delete_post(
        ctx: Context<DeletePost>
    ) -> Result<()> {
        Ok(())
    }
    
    pub fn create_comment(
        ctx: Context<CreateComment>,
        body: String
    ) -> Result<()> {
        Ok(())
    }

}




#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer=signer, space=10000, seeds=[b"PROFILE_STATE".as_ref(), signer.key().as_ref()], bump)]
    pub profile: Account<'info, states::Profile>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMembershipPlan<'info> {
    #[account(init, payer=signer, space=10000, seeds=[b"MEMBERSHIP_PLAN_STATE".as_ref(), signer.key().as_ref()], bump)]
    pub membership_plan: Account<'info, states::MembershipPlan>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct BuyMembership<'info> {
    #[account(mut)]
    pub membership_plan: Account<'info, states::MembershipPlan>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer=signer, space=10000, seeds=[b"POST".as_ref(), signer.key().as_ref()], bump)]
    pub post : Account<'info, states::Post>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(mut, has_one = authority, close = authority)]
    pub post: Account<'info, states::Post>,
    
    #[account(mut)]
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct CreateComment<'info> {
    #[account(init, payer=signer, space = 1000, seeds=[b"COMMENT".as_ref(), signer.key().as_ref()], bump)]

    pub comment: Account<'info, states::Comment>,
    
    #[account()]
    pub post: Account<'info, states::Post>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteComment<'info> {
    #[account(mut, has_one=authority, close=authority)]
    pub comment: Account<'info, states::Comment>,

    #[account(mut)]
    pub authority: Signer<'info>
}
