use super::errors::ErrorCode;
use super::*;

pub fn voting_for(
    ctx: Context<VoteForTournament>,
    _team_name: String,
    _team_id: u64,
    vote_type: VoteType,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    require!(
        team.members.contains(ctx.accounts.signer.key),
        ErrorCode::MemberNotInTeamError
    );

    require!(
        !team.voted_players.contains(ctx.accounts.signer.key),
        ErrorCode::AlreadyVotedError
    );

    match vote_type {
        VoteType::Yes => {
            team.voted_players.push(*ctx.accounts.signer.key);
            team.yes_votes += 1;
        }
        VoteType::No => {
            team.voted_players.push(*ctx.accounts.signer.key);
        }
    }

    if team.yes_votes > 2 {
        team.yes_votes = 0;
        team.voted_players = vec![];

        team.voting_result = true;
    }

    msg!(
        "{} is successfully voted for the tournament {}",
        team.name,
        team.name
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct VoteForTournament<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}