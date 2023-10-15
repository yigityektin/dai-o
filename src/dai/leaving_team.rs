use super::errors::ErrorCode;
use super::*;

pub fn leaving_team(ctx: Context<LeaveTeam>, _team_name: String, _team_id: u64) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    require!(
        team.members.contains(ctx.accounts.signer.key),
        ErrorCode::MemberNotInTeamError
    );

    if team.captain == *ctx.accounts.signer.key {
        team.captain = team.members[1];
    }

    team.members.retain(|&x| x != *ctx.accounts.signer.key);

    msg!(
        "{} is successfully removed from the team {}",
        ctx.accounts.signer.key,
        team.name
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct LeaveTeam<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}