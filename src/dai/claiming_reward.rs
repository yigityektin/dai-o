use super::errors::ErrorCode;
use super::*;

pub fn claiming_reward(
    ctx: Context<ClaimReward>,
    _team_name: String,
    _team_id: u64,
    reward: u64,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    require!(
        team.members.contains(ctx.accounts.to.key),
        ErrorCode::MemberNotInTeamError
    );

    let index = team
        .members
        .iter()
        .position(|&r| r == *ctx.accounts.to.key)
        .unwrap();

    let expected_max_reward = team.prize * team.distribution_percentages[index] as u64 / 100;

    require!(
        reward <= expected_max_reward,
        ErrorCode::InvalidPercentageError
    );

    let from = ctx.accounts.from.to_account_info();
    let to = ctx.accounts.to.to_account_info();

    **from.try_borrow_mut_lamports()? -= reward;
    **to.try_borrow_mut_lamports()? += reward;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct ClaimReward<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,
    #[account(mut)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account()]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}