use super::Db;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_sqldb::{minicbor, SqlDb, SqlDbError};

const TABLE_VETS: &str = "vets";

#[derive(Serialize, Deserialize, minicbor::Decode, Clone)]
pub(crate) struct DbVet {
    #[n(0)]
    pub id: u64,
    #[n(1)]
    pub firstname: String,
    #[n(2)]
    pub lastname: String,
    /// Comma-delimited list of specialties
    #[n(3)]
    pub specialties: String,
}

pub(crate) async fn list_vets(ctx: &Context, client: &Db) -> Result<Vec<DbVet>, SqlDbError> {
    let resp = client
        .fetch(
            ctx,
            &format!(
                "select id, firstname, lastname, specialties from {} order by lastname, firstname",
                TABLE_VETS
            ),
        )
        .await?;
    let rows: Vec<DbVet> = minicbor::decode(&resp.rows)?;
    Ok(rows)
}

impl From<DbVet> for petclinic_interface::Vet {
    fn from(source: DbVet) -> Self {
        petclinic_interface::Vet {
            first_name: source.firstname,
            id: source.id,
            last_name: source.lastname,
            specialties: source
                .specialties
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        }
    }
}
