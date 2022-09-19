use crate::domain::Seating;
use crate::popis_error::Result;

use super::provider::Provider;

const SEATING: &str = "seating";

pub async fn random_seating(provider: &Provider) -> Result<Seating> {
    provider.client.query_one(statement, params)
}