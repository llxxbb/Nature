use crate::common::Result;
use crate::db::{D_M, MetaDao, RawMeta};
use crate::util::*;

pub struct MetaService {}

impl MetaService {
    pub async fn id_great_than(from: i32, limit: i32) -> Result<Vec<RawMeta>> {
        let limit = if limit < *QUERY_SIZE_LIMIT {
            limit
        } else { *QUERY_SIZE_LIMIT };
        D_M.id_great_than(from, limit).await
    }
}