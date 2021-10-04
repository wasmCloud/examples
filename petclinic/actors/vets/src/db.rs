use super::Db;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_sqldb::{FetchResult, SqlDb, SqlDbError, minicbor};

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
    let rows: Vec<DbVet> = safe_decode(&resp)?;
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

/// When using this to decode Vecs, will get an empty vec
/// as a response when no rows are returned
fn safe_decode<'b, T>(resp: &'b FetchResult) -> Result<T, minicbor::decode::Error>
where
    T: minicbor::Decode<'b> + Default,
{
    if resp.num_rows == 0 {
        Ok(T::default())
    } else {
        minicbor::decode(&resp.rows)
    }
}