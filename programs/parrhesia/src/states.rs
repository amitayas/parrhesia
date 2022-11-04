use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Profile {
    pub authority: Pubkey,
    pub name: String, 
    pub bio: String
}

#[account]
#[derive(Default)]
pub struct MembershipPlan {
    pub authority: Pubkey,
    pub name: String,
    pub description: String,
    pub amount: u64,
    pub count: u64
}

#[account]
#[derive(Default)]
pub struct Post {
    pub authority: Pubkey,
    pub body: String,
}

#[account]
#[derive(Default)]
pub struct Comment {
    pub authority: Pubkey,
    pub body: String,
    pub post: Pubkey
}


