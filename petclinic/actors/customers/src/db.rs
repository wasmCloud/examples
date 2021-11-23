// Philosophical note:
//
// Opinion here is that the creation of tables and execution of migrations is something that
// should be done by infrastructure layers. In other words, actors should not be responsible
// for creating the tables upon which they rely. One of the main reasons for this is that
// an area where relational/sql DBs tend to diverge in syntax is in the CREATE TABLE syntax,
// making it very easy to accidentally tightly couple an actor to a particular db implementation
// by using niche syntax during the create table phase.
//
// Further, just because a provider implements the wasmcloud:sqldb interface does not mean that
// provider supports creation of tables through SQL statements. Yet another concern is that in
// production environments, the db connection string used by the actors will typically be restricted
// to only being able to read/write a subset of tables -- production environments should never allow
// application code security privilege to create/drop tables.

use super::Db;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_sqldb::{minicbor, FetchResult, SqlDb, SqlDbError};

const TABLE_OWNERS: &str = "owners";
const TABLE_PETS: &str = "pets";
const TABLE_PETTYPES: &str = "pettypes";

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

/// Represents a row in the owners table
#[derive(Serialize, Deserialize, minicbor::Decode, Clone)]
pub(crate) struct Owner {
    #[n(0)]
    pub id: u64,
    #[n(1)]
    pub address: String,
    #[n(2)]
    pub city: String,
    #[n(3)]
    pub email: String,
    #[n(4)]
    pub firstname: String,
    #[n(5)]
    pub lastname: String,
    #[n(6)]
    pub telephone: String,
}

/// Represents a row in the pets table
#[derive(Serialize, Deserialize, minicbor::Decode, Clone)]
pub(crate) struct Pet {
    #[n(0)]
    pub id: u64,
    #[n(1)]
    pub pettype: u64,
    #[n(2)]
    pub name: String,
    #[n(3)]
    pub bday: u32,
    #[n(4)]
    pub bmonth: u32,
    #[n(5)]
    pub byear: u32,
    #[n(6)]
    pub ownerid: u64,
}

/// Represents a row in the pettypes table
#[derive(Serialize, Deserialize, minicbor::Decode, Clone)]
pub(crate) struct PetType {
    #[n(0)]
    pub id: u64,
    #[n(1)]
    pub name: String,
}

/// Deletes a pet with a given ID
pub(crate) async fn delete_pet(ctx: &Context, client: &Db, id: u64) -> Result<(), SqlDbError> {
    let resp = client
        .execute(
            ctx,
            &format!(
                r#"
            delete from {} where id = {}
            "#,
                TABLE_PETS, id
            ),
        )
        .await?;
    match resp.error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}

/// Adds a pet to a given owner
pub(crate) async fn add_pet(
    ctx: &Context,
    client: &Db,
    owner_id: u64,
    pet: &petclinic_interface::Pet,
) -> Result<(), SqlDbError> {
    let pet = Pet {
        ownerid: owner_id,
        ..Pet::try_from(pet.clone()).map_err(|_| SqlDbError::new("parse", "Invalid pet".into()))?
    };

    let resp = client
        .execute(
            ctx,
            &format!(
                r#"
            insert into {} (id, pettype, name, bday, bmonth, byear, ownerid)
            values({}, {}, '{}', {}, {}, {}, {})
            "#,
                TABLE_PETS,
                pet.id,
                pet.pettype,
                pet.name,
                pet.bday,
                pet.bmonth,
                pet.byear,
                pet.ownerid
            ),
        )
        .await?;
    match resp.error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}

/// Creates a new owner/customer
pub(crate) async fn create_owner(
    ctx: &Context,
    client: &Db,
    arg: &petclinic_interface::Owner,
) -> Result<(), SqlDbError> {
    let owner = Owner::try_from(arg.clone())
        .map_err(|_e| SqlDbError::new("parse", "Invalid new owner".into()))?;
    let resp = client
        .execute(
            ctx,
            &format!(
                r#"
                insert into {} (id, address, city, email, firstname, lastname, telephone)
                values({}, '{}', '{}', '{}', '{}', '{}', '{}')
                "#,
                TABLE_OWNERS,
                arg.id,
                owner.address,
                owner.city,
                owner.email,
                owner.firstname,
                owner.lastname,
                owner.telephone,
            ),
        )
        .await?;
    match resp.error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}

/// Locates an owner by ID
pub(crate) async fn find_owner(
    ctx: &Context,
    client: &Db,
    id: u64,
) -> Result<Option<Owner>, SqlDbError> {
    let resp = client
        .fetch(
            ctx,
            &format!(
                "select id, address, city, email, firstname, lastname, telephone from {} where id \
                 = {}",
                TABLE_OWNERS, id
            ),
        )
        .await?;

    if resp.rows.is_empty() {
        Ok(None)
    } else {
        let rows: Vec<Owner> = safe_decode(&resp)?;
        Ok(Some(rows.get(0).cloned().unwrap())) // this unwrap is safe, we know we have at least a row
    }
}

/// Locates a pet by ID (does not utilize the owner ID)
pub(crate) async fn find_pet(
    ctx: &Context,
    client: &Db,
    id: u64,
) -> Result<Option<Pet>, SqlDbError> {
    let resp = client
        .fetch(
            ctx,
            &format!(
                "select id, pettype, name, bday, bmonth, byear, ownerid from {} where id = {}",
                TABLE_PETS, id
            ),
        )
        .await?;

    if resp.rows.is_empty() {
        Ok(None)
    } else {
        let rows: Vec<Pet> = safe_decode(&resp)?;
        Ok(Some(rows.get(0).cloned().unwrap()))
    }
}

/// Lists all owners/customers in the database
pub(crate) async fn list_all_owners(ctx: &Context, client: &Db) -> Result<Vec<Owner>, SqlDbError> {
    let resp = client
        .fetch(
            ctx,
            &format!(
                "select id, address, city, email, firstname, lastname, telephone from {}",
                TABLE_OWNERS
            ),
        )
        .await?;

    let rows: Vec<Owner> = safe_decode(&resp)?;
    Ok(rows)
}

/// Lists all known pet types
pub(crate) async fn list_all_pet_types(
    ctx: &Context,
    client: &Db,
) -> Result<Vec<PetType>, SqlDbError> {
    let resp = client
        .fetch(ctx, &format!("select id, name FROM {}", TABLE_PETTYPES))
        .await?;
    let rows: Vec<PetType> = safe_decode(&resp)?;
    Ok(rows)
}

/// Lists all pets owned by a specific customer
pub(crate) async fn list_pets_by_owner(
    ctx: &Context,
    client: &Db,
    owner_id: u64,
) -> Result<Vec<Pet>, SqlDbError> {
    let resp = client
        .fetch(
            ctx,
            &format!(
                "select id, pettype, name, bday, bmonth, byear, ownerid from {}
                where ownerid = {}",
                TABLE_PETS, owner_id
            ),
        )
        .await?;

    let rows: Vec<Pet> = safe_decode(&resp)?;
    Ok(rows)
}

/// Updates a pet based on its unique ID. Note that this code
/// doesn't prevent moving a pet from one owner to another - that's
/// a business logic decision best left to higher level functions
pub(crate) async fn update_pet(
    ctx: &Context,
    client: &Db,
    arg: &petclinic_interface::Pet,
) -> Result<(), SqlDbError> {
    let pet =
        Pet::try_from(arg.clone()).map_err(|_e| SqlDbError::new("parse", "Invalid pet".into()))?;

    let resp = client
        .execute(
            ctx,
            &format!(
                r#"
            update {} 
                SET pettype = {},
                    name = '{}',
                    bday = {},
                    bmonth = {},
                    byear = {},
                    ownerid = {}
            WHERE id = {}
            "#,
                TABLE_PETS,
                pet.pettype,
                pet.name,
                pet.bday,
                pet.bmonth,
                pet.byear,
                pet.ownerid,
                pet.id
            ),
        )
        .await?;
    match resp.error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}

/// Updates an existing owner/customer
pub(crate) async fn update_owner(
    ctx: &Context,
    client: &Db,
    arg: &petclinic_interface::Owner,
) -> Result<(), SqlDbError> {
    let owner = Owner::try_from(arg.clone())
        .map_err(|_e| SqlDbError::new("parse", "Invalid owner".into()))?;

    let resp = client
        .execute(
            ctx,
            &format!(
                r#"
                update {}
                SET address = '{}', city = '{}', email = '{}',
                firstname = '{}', lastname = '{}', telephone = '{}'
                WHERE id = {}                
                "#,
                TABLE_OWNERS,
                owner.address,
                owner.city,
                owner.email,
                owner.firstname,
                owner.lastname,
                owner.telephone,
                owner.id,
            ),
        )
        .await?;

    match resp.error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}

impl From<PetType> for petclinic_interface::PetType {
    fn from(pt: PetType) -> Self {
        petclinic_interface::PetType {
            id: pt.id,
            name: pt.name,
        }
    }
}

impl From<Owner> for petclinic_interface::Owner {
    fn from(o: Owner) -> Self {
        petclinic_interface::Owner {
            id: o.id,
            address: if o.address.is_empty() {
                None
            } else {
                Some(o.address.clone())
            },
            city: if o.city.is_empty() {
                None
            } else {
                Some(o.city)
            },
            email: o.email,
            first_name: o.firstname,
            last_name: if o.lastname.is_empty() {
                None
            } else {
                Some(o.lastname)
            },
            telephone: if o.telephone.is_empty() {
                None
            } else {
                Some(o.telephone)
            },
        }
    }
}

impl From<Pet> for petclinic_interface::Pet {
    fn from(source: Pet) -> Self {
        petclinic_interface::Pet {
            birthdate: petclinic_interface::Date {
                day: source.bday as _,
                month: source.bmonth as _,
                year: source.byear as _,
            },
            id: source.id,
            name: source.name,
            pet_type: source.pettype,
        }
    }
}

impl TryFrom<petclinic_interface::Pet> for Pet {
    type Error = std::io::Error;

    fn try_from(arg: petclinic_interface::Pet) -> Result<Self, Self::Error> {
        check_safety("name", &arg.name)?;
        let pet = Pet {
            id: arg.id,
            name: arg.name,
            pettype: arg.pet_type,
            bday: arg.birthdate.day as _,
            byear: arg.birthdate.year as _,
            bmonth: arg.birthdate.month as _,
            ownerid: 0,
        };
        Ok(pet)
    }
}

impl TryFrom<petclinic_interface::Owner> for Owner {
    type Error = std::io::Error;

    fn try_from(arg: petclinic_interface::Owner) -> Result<Self, Self::Error> {
        let address = arg.address.unwrap_or_else(|| "".into());
        let city = arg.city.unwrap_or_else(|| "".into());
        let lastname = arg.last_name.unwrap_or_else(|| "".into());
        let telephone = arg.telephone.unwrap_or_else(|| "".into());

        check_safety("address", &address)?;
        check_safety("city", &city)?;
        check_safety("email", &arg.email)?;
        check_safety("firstname", &arg.first_name)?;
        check_safety("lastname", &lastname)?;
        check_safety("telephone", &telephone)?;

        let owner = Owner {
            id: arg.id,
            address,
            city,
            email: arg.email,
            firstname: arg.first_name,
            lastname,
            telephone,
        };
        Ok(owner)
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
