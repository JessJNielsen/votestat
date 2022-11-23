use crate::database::DB;
use crate::repository::Repository;

pub mod create_super_district;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
}

impl Service {
    pub fn new(db: DB) -> Service {
        let repo = Repository::new();

        Service { db, repo }
    }
}
