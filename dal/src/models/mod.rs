use async_trait::async_trait;

pub mod district;
pub mod super_district;

#[async_trait]
pub trait Insert<Input> {
    type Output;

    async fn insert(input: &Input) -> anyhow::Result<Option<Self::Output>>;
}

#[async_trait]
trait Update<Input> {
    type Output;

    async fn update(input: Input) -> Self::Output;
}

#[async_trait]
trait Delete<Input> {
    type Output;

    async fn delete(input: Input) -> Self::Output;
}

// #[async_trait]
// trait Get<Rhs = Self> {
//     type Output;
//
//     async fn get(self, rhs: Rhs) -> Self::Output;
// }

// Plan

// Table and entities to make

// Party
// has id and name of party

// Super District
// has id and name

// District
// has id and name and id of super district

// Voting Center
// has id and name and id of district

// Candidate
// has id and name and id of party

// IndependentCandidate
// Same as a party but only in one super district

// PartyVotesVotingCentre
// Each row has id of party and of voting centre and amount of votes

// CandidateVotesVotingCentre
// Each row has id of candidate and of voting centre and amount of votes
