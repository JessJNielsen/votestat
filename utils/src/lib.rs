use domain::elections::ElectionService;
use domain::voting_districts::VotingDistrictService;

pub struct Context {
    pub election_service: ElectionService,
    pub voting_district_service: VotingDistrictService,
}
