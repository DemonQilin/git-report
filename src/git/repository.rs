use std::path::Path;

use super::error;
use super::models::Commit;

pub trait GitRepository {
    fn get_all_commits<'a>(&self, path: &'a Path) -> error::Result<'a, Vec<Commit>>;
}
