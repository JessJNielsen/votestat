use chrono::NaiveDateTime;

use crate::database::connect_database;

#[derive(Debug)]
pub struct SuperDistrictInput {
    pub name: String,
    pub sub_districts: Vec<District>,
}

#[derive(Debug)]
pub struct SuperDistrictEntity {
    pub super_district_id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl SuperDistrictEntity {
    pub async fn insert(input: &SuperDistrictInput) -> anyhow::Result<Option<SuperDistrictEntity>> {
        let new_super_district: Option<SuperDistrictEntity> = sqlx::query_as!(
            SuperDistrictEntity,
            r#"
                INSERT OR IGNORE INTO super_districts (name)
                VALUES ($1)
                RETURNING
                super_district_id as "super_district_id!",
                name as "name!",
                created_at as "created_at!"
            "#,
            input.name,
        )
        .fetch_optional(&connect_database().await)
        .await?;

        println!("new_super_district {:?}", &new_super_district);

        Ok(new_super_district)
    }
}

#[derive(Debug)]
pub struct District {
    pub name: String,
    pub link: String,
}

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
