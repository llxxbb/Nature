use crate::common::Result;
use crate::db::{D_R, RawRelation, RelationDao};
use crate::util::*;

pub struct RelationService {}

impl RelationService {
    pub async fn id_great_than(from: i32, limit: i32) -> Result<Vec<RawRelation>> {
        let limit = if limit < *QUERY_SIZE_LIMIT {
            limit
        } else { *QUERY_SIZE_LIMIT };
        D_R.id_great_than(from, limit).await
    }
}