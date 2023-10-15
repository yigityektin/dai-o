use super::errors::ErrorCode;
use super::*;

pub fn removing_member(
    ctx: Context<RemoveMember>,
    _team_name: String,
    _team_id: u64,
    member: Pubkey,
) -> Result<()> {

    msg!(
        "{} is successfully removed from the team {}",
        member,
        team.name
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct RemoveMember<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}