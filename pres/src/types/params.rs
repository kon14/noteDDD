use serde::Deserialize;
use utoipa::IntoParams;

use common::params as cmn;

#[derive(Deserialize, IntoParams)]
pub struct PaginationParams {
    pub skip: u32,
    pub limit: u32,
}

impl From<PaginationParams> for cmn::PaginationParams {
    fn from(pagination: PaginationParams) -> Self {
        Self {
            skip: pagination.skip,
            limit: pagination.limit,
        }
    }
}
