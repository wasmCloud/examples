use super::Db;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_sqldb::{minicbor, QueryResult, SqlDb, SqlDbError, Statement};

const TABLE_VISITS: &str = "visits";
const PETCLINIC_DB: &str = "petclinic";

static REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^[-a-zA-Z0-9 ,._/@]+$").unwrap());

fn check_safety(tag: &str, uncertain_input: &str) -> Result<(), std::io::Error> {
    if !REGEX.is_match(uncertain_input) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{} contains invalid characters", tag),
        ));
    }
    Ok(())
}

#[derive(Serialize, Deserialize, minicbor::Decode, Clone)]
pub(crate) struct DbVisit {
    #[n(0)]
    pub day: u32,
    #[n(1)]
    pub month: u32,
    #[n(2)]
    pub year: u32,
    #[n(3)]
    pub description: String,
    #[n(4)]
    pub petid: u64,
    #[n(5)]
    pub vetid: u64,
    #[n(6)]
    pub ownerid: u64,
    #[n(7)]
    pub hour: u32,
    #[n(8)]
    pub minute: u32,
}

pub(crate) async fn list_visits_by_owner_and_pet(
    ctx: &Context,
    client: &Db,
    owner_id: u64,
    pet_ids: Option<Vec<u64>>,
) -> Result<Vec<DbVisit>, SqlDbError> {
    let sql = if let Some(petids) = pet_ids {
        let petids = format!(
            "({})",
            petids
                .iter()
                .map(|pid| pid.to_string())
                .collect::<Vec::<_>>()
                .join(",")
        );
        format!(
            "select day, month, year, description, petid, vetid, ownerid, hour, minute from {} \
             where ownerid = {} AND petid IN {}",
            TABLE_VISITS, owner_id, petids
        )
    } else {
        format!(
            "select day, month, year, description, petid, vetid, ownerid, hour, minute from {} \
             where ownerid = {}",
            TABLE_VISITS, owner_id
        )
    };

    let resp = client
        .query(
            ctx,
            &Statement {
                sql,
                database: Some(PETCLINIC_DB.to_string()),
                ..Default::default()
            },
        )
        .await?;

    let rows: Vec<DbVisit> = safe_decode(&resp)?;
    Ok(rows)
}

pub(crate) async fn record_visit(
    ctx: &Context,
    client: &Db,
    owner_id: u64,
    visit: petclinic_interface::Visit,
) -> Result<(), SqlDbError> {
    check_safety("description", &visit.description)
        .map_err(|e| SqlDbError::new("invalid", format!("{}", e)))?;

    let resp = client
        .execute(
            ctx,
            &Statement {
                sql: format!(
                    r#"
            insert into {} (day, month, year, description, petid, vetid, ownerid, hour, minute)
            values({}, {}, {}, '{}', {}, {}, {}, {}, {})
            "#,
                    TABLE_VISITS,
                    visit.date.day,
                    visit.date.month,
                    visit.date.year,
                    visit.description,
                    visit.pet_id,
                    visit.vet_id,
                    owner_id,
                    visit.time.hour,
                    visit.time.minute
                ),
                database: Some(PETCLINIC_DB.to_string()),
                ..Default::default()
            },
        )
        .await?;

    match resp.error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}

impl From<DbVisit> for petclinic_interface::Visit {
    fn from(source: DbVisit) -> Self {
        petclinic_interface::Visit {
            date: petclinic_interface::Date {
                day: source.day as _,
                month: source.month as _,
                year: source.year as _,
            },
            description: source.description,
            pet_id: source.petid,
            time: petclinic_interface::Time {
                hour: source.hour as _,
                minute: source.minute as _,
            },
            vet_id: source.vetid,
        }
    }
}

/// When using this to decode Vecs, will get an empty vec
/// as a response when no rows are returned
fn safe_decode<'b, T>(resp: &'b QueryResult) -> Result<T, minicbor::decode::Error>
where
    T: minicbor::Decode<'b> + Default,
{
    if resp.num_rows == 0 {
        Ok(T::default())
    } else {
        minicbor::decode(&resp.rows)
    }
}