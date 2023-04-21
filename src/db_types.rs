use chrono::{DateTime, Utc};
use sqlx::prelude::*;
use sqlx::PgExecutor;
use sqlx::PgPool;

use crate::api_types;

type Varchar = String;
type Serial = i32;
type Real = f32;
type Timestamptz = DateTime<Utc>;

// macro_rules! Insert {
//     (@insert $self:ident $t:ty; $e: expr; $($f:ident,)+) => {
//         sqlx::query_as!(
//             Self,
//             $e,
//             $($self.$f,)+
//         )
//     };
//     ($t:ty; $($f:ident,)+) => {
//         impl $t {
//             pub async fn insert(self, exec: &PgPool) -> Result<Self, sqlx::Error> {
//                 paste::paste!(
//                     Insert!(@insert self $t;
//                             stringify!(hello world)
//                             ; $($f,)+)
//                     )
//                     // "INSERT INTO ObjectType (" $($f,)+ ") VALUES ($1,$2,  $3, $4, $5) RETURNING id, name, ky, description, locale_key, deleted;"
//                 .fetch_one(exec).await
//             }
//         }
//     };
// }

#[derive(FromRow, Debug, Clone)]
pub struct ObjectType {
    pub id: Serial,
    pub name: Varchar,
    pub ky: Varchar,
    pub description: Varchar,
    pub locale_key: Varchar,
    pub deleted: bool,
}

impl From<api_types::ObjectType> for ObjectType {
    fn from(
        api_types::ObjectType {
            id,
            name,
            key,
            description,
            locale_key,
            deleted,
        }: api_types::ObjectType,
    ) -> Self {
        ObjectType {
            id,
            name,
            ky: key,
            description,
            locale_key,
            deleted,
        }
    }
}

// Insert!(ObjectType; name, ky, description, locale_key, deleted,);
impl ObjectType {
    pub async fn insert<'a, B>(self, exec: &'a B) -> Result<Self, sqlx::Error>
    where
        &'a B: PgExecutor<'a> + 'a,
    {
        sqlx::query_as!(
            Self,
            r#"
                INSERT INTO ObjectType
                (name, ky, description, locale_key, deleted)
                VALUES
                ($1, $2, $3, $4, $5)
                RETURNING
                id, name, ky, description, locale_key, deleted;"#,
            self.name,
            self.ky,
            self.description,
            self.locale_key,
            self.deleted
        )
        .fetch_one(exec)
        .await
    }
}

#[derive(FromRow, Debug, Clone)]
pub struct AttributeType {
    pub id: Serial,
    pub name: Varchar,
    pub ky: Varchar,
    pub locale_key: Varchar,
    pub datatype: Varchar,
}

impl AttributeType {
    pub async fn insert<'a, B>(self, exec: &'a B) -> Result<Self, sqlx::Error>
    where
        &'a B: PgExecutor<'a> + 'a,
    {
        sqlx::query_as!(
            Self,
            r#"
                INSERT INTO AttributeType
                (name, ky, locale_key, datatype)
                VALUES
                ($1, $2, $3, $4)
                RETURNING
                id, name, ky, locale_key, datatype;"#,
            self.name,
            self.ky,
            self.locale_key,
            self.datatype
        )
        .fetch_one(exec)
        .await
    }
}

#[derive(FromRow, Debug, Clone)]
pub struct Attribute {
    pub id: Serial,
    pub ty: Varchar,
    pub json_data: Varchar,
}

impl Attribute {
    pub async fn insert<'a, B>(self, exec: &'a B) -> Result<Self, sqlx::Error>
    where
        &'a B: PgExecutor<'a> + 'a,
    {
        sqlx::query_as!(
            Self,
            r#" INSERT INTO Attribute
                (ty, json_data)
                VALUES
                ($1, $2)
                RETURNING
                id, ty, json_data"#,
            self.ty,
            self.json_data,
        )
        .fetch_one(exec)
        .await
    }
}

#[derive(FromRow, Debug, Clone)]
pub struct Entity {
    pub id: Serial,

    // #[sqlx(default)]
    // pub parent_id: Option<Serial>,
    pub latitude_position: Real,
    pub longitude_position: Real,

    pub created_by: Varchar,
    pub edited_by: Varchar,

    pub created_at: Timestamptz,
    pub edited_at: Timestamptz,

    pub geo_json: Varchar,
    pub prop_json: Varchar,
    pub ky: Varchar,
    pub ty: Serial,
    pub deleted: bool,
}

impl Entity {
    pub async fn insert<'a, B>(self, exec: &'a B) -> Result<Self, sqlx::Error>
    where
        &'a B: PgExecutor<'a> + 'a,
    {
        sqlx::query_as!(
                Self,
                r#" INSERT INTO Entity
                (latitude_position, longitude_position, created_by, edited_by, created_at, edited_at, geo_json, prop_json, ky, ty, deleted)
                VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING
                id, latitude_position, longitude_position, created_by, edited_by, created_at, edited_at, geo_json, prop_json, ky, ty, deleted "#,
                // parent,
                self.latitude_position,
                self.longitude_position,
                self.created_by,
                self.edited_by,
                self.created_at,
                self.edited_at,
                self.geo_json,
                self.prop_json,
                self.ky,
                self.ty,
                self.deleted
                )
                .fetch_one(exec)
                .await
    }
}

pub struct EntityAttributeJunction {
    pub id: Serial,
    pub entity_id: Serial,
    pub attribute_id: Serial,
}

impl EntityAttributeJunction {
    pub async fn insert<'a, B>(self, exec: &'a B) -> Result<Self, sqlx::Error>
    where
        &'a B: PgExecutor<'a> + 'a,
    {
        sqlx::query_as!(
            Self,
            r#" INSERT INTO EntityAttributesJunction
                (entity_id, attribute_id)
                VALUES
                ($1, $2)
                RETURNING
                id, entity_id, attribute_id"#,
            self.entity_id,
            self.attribute_id,
        )
        .fetch_one(exec)
        .await
    }
}
