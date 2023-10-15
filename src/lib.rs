use anchor_lang::prelude::*;
use team::*;

pub mod team;

declare_id!("");

#[program]
pub mod dai_dao {

    use super::*;

    use team::can_join::{can_join, CanJoinTournament};
    use team::claiming_reward::{claiming_reward, ClaimReward};
    use team::handle_distribute_proposal::{
        handle_distribute_proposal, DistributionProposalHandler,
    };
    use team::initing_percentage_proposal::{initing_percentage_proposal, InitPercentageProposal};
    use team::leaving_team::{leaving_team, LeaveTeam};
    use team::removing_member::{removing_member, RemoveMember};
    use team::voting_for::{voting_for_tournament, VoteForTournament};
    use team::VoteType;

    // ----------------------------------------------

    // removing member
    pub fn remove_member(
        ctx: Context<RemoveMember>,
        _team_name: String,
        _team_id: u64,
        member: Pubkey,
    ) -> Result<()> {
        return removing_member(ctx, _team_name, _team_id, member);
    }

    // leaving team
    pub fn leave_team(ctx: Context<LeaveTeam>, _team_name: String, _team_id: u64) -> Result<()> {
        return leaving_team(ctx, _team_name, _team_id);
    }

    // vote
    pub fn vote_for(
        ctx: Context<VoteForTournament>,
        _team_name: String,
        _team_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        return voting_for(ctx, _team_name, _team_id, vote_type);
    }

    // init percentage proposal
    pub fn init_percentage_proposal(
        ctx: Context<InitPercentageProposal>,
        _team_name: String,
        _team_id: u64,
        percentages: Vec<u8>,
    ) -> Result<()> {
        return initing_percentage_proposal(ctx, _team_name, _team_id, percentages);
    }

    // reward distribution proposal handler
    pub fn distribution_proposal_handler(
        ctx: Context<DistributionProposalHandler>,
        _team_name: String,
        _team_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        return handle_distribute_proposal(ctx, _team_name, _team_id, vote_type);
    }
    // two functions above will basically be used to vote for the distribution of the rewards

    // can join
    pub fn can_join(
        ctx: Context<CanJoinTournament>,
        _team_name: String,
        _team_id: u64,
    ) -> Result<()> {
        return can_join(ctx, _team_name, _team_id);
    }

    // distribute rewards
    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        _team_name: String,
        _team_id: u64,
        reward: u64,
    ) -> Result<()> {
        return claiming_reward(ctx, _team_name, _team_id, reward);
    }
}