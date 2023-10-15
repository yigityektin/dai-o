use super::errors::ErrorCode;
use super::*;

pub fn initing_percentage_proposal(
    ctx: Context<InitPercentageProposal>,
    _team_name: String,
    _team_id: u64,
    percentages: Vec<u8>,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    let sum: u8 = percentages.iter().sum();

    require!(sum == 100, ErrorCode::InvalidPercentageError);

    require!(
        team.active_tournament != Pubkey::default(),
        ErrorCode::NoActiveTournamentError
    );

    require!(
        team.captain == *ctx.accounts.signer.key,
        ErrorCode::NotCaptainError
    );

    team.distribution_percentages = percentages;

    msg!(
        "{} is successfully proposed a percentage {:?}",
        team.name,
        team.distribution_percentages
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct InitPercentageProposal<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}