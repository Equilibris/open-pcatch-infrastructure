use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db_types;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "attributeDataType")]
pub enum DataType {
    DateType {
        #[serde(with = "chrono::serde::ts_seconds")]
        attribute_value: DateTime<Utc>,
    },
    IntegerType {
        attribute_value: i64,
    },
    FloatType {
        attribute_value: f64,
    },
    StringType {
        attribute_value: String,
    },
    TextType {
        attribute_value: String,
    },
}

#[derive(thiserror::Error, Debug)]
enum DataTypeDeserializeError {
    #[error("Data typename is invalid")]
    InvalidTypename,

    #[error("Serde failed to deserialize")]
    SerdeDeserializeError(#[from] serde_json::error::Error),
}

impl DataType {
    fn into_type_and_data(self) -> (&'static str, String) {
        let (a, b) = match self {
            DataType::DateType { attribute_value } => {
                ("DATE_TYPE", serde_json::to_string(&attribute_value))
            }
            DataType::IntegerType { attribute_value } => {
                ("INTEGER_TYPE", serde_json::to_string(&attribute_value))
            }
            DataType::FloatType { attribute_value } => {
                ("FLOAT_TYPE", serde_json::to_string(&attribute_value))
            }
            DataType::StringType { attribute_value } => {
                ("STRING_TYPE", serde_json::to_string(&attribute_value))
            }
            DataType::TextType { attribute_value } => {
                ("TEXT_TYPE", serde_json::to_string(&attribute_value))
            }
        };
        (a, b.expect("Failed to serialize"))
    }
    fn from_type_and_data(
        ty: impl AsRef<str>,
        data: impl AsRef<str>,
    ) -> Result<Self, DataTypeDeserializeError> {
        use serde_json::de::from_str;
        Ok(match ty.as_ref() {
            "DATE_TYPE" => Self::DateType {
                attribute_value: from_str(data.as_ref())?,
            },
            "INTEGER_TYPE" => Self::IntegerType {
                attribute_value: from_str(data.as_ref())?,
            },
            "FLOAT_TYPE" => Self::FloatType {
                attribute_value: from_str(data.as_ref())?,
            },
            "STRING_TYPE" => Self::StringType {
                attribute_value: from_str(data.as_ref())?,
            },
            "TEXT_TYPE" => Self::TextType {
                attribute_value: from_str(data.as_ref())?,
            },
            _ => return Err(DataTypeDeserializeError::InvalidTypename),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeType {
    pub id: i32,

    pub name: String,
    pub key: String,
    pub locale_key: String,

    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub id: i32,

    pub name: String,
    pub key: String,
    pub locale_key: String,

    #[serde(flatten)]
    pub data: DataType,
}

impl Attribute {
    fn into_db(self) -> (db_types::Attribute, db_types::AttributeType) {
        let Self {
            id: _,
            name,
            key,
            locale_key,
            data,
        } = self;

        let (_, data) = data.into_type_and_data();

        (
            db_types::Attribute {
                id: 0,
                ty: name.to_string(),
                json_data: data,
            },
            db_types::AttributeType {
                id: 0,
                name: name.to_string(),
                ky: key,
                locale_key,
                datatype: name.to_string(),
            },
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeoJson {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropJson {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectEntity {
    pub parent_id: i32,

    pub latitude_position: f32,
    pub longitude_position: f32,

    pub created_by: String,
    pub edited_by: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub edited_at: DateTime<Utc>,

    // TODO: custom Serialize for geojson
    pub geo_json: GeoJson,
    pub search_string: String,
    // TODO: same as geo
    pub prop_json: PropJson,

    pub key: String,

    #[serde(rename = "type")]
    pub ty: ObjectType,
    pub attributes: Vec<Attribute>,
}

impl ObjectEntity {
    pub fn into_db(
        self,
    ) -> (
        db_types::Entity,
        db_types::ObjectType,
        Vec<(db_types::Attribute, db_types::AttributeType)>,
    ) {
        let Self {
            parent_id,
            latitude_position,
            longitude_position,
            created_by,
            edited_by,
            created_at,
            edited_at,
            geo_json,
            search_string,
            prop_json,
            key,
            ty,
            attributes,
        } = self;
        use db_types::*;
        (
            Entity {
                id: 0,
                latitude_position,
                longitude_position,
                created_by,
                edited_by,
                created_at,
                edited_at,
                geo_json: "".to_string(),
                prop_json: "".to_string(),
                ky: key,
                ty: 0,
                deleted: false,
            },
            ty.into(),
            attributes
                .into_iter()
                .map(self::Attribute::into_db)
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub description: String,
    pub locale_key: String,
    pub deleted: bool,
}

impl From<db_types::ObjectType> for ObjectType {
    fn from(
        db_types::ObjectType {
            id,
            name,
            ky,
            description,
            locale_key,
            deleted,
        }: db_types::ObjectType,
    ) -> Self {
        Self {
            id,
            name,
            key: ky,
            description,
            locale_key,
            deleted,
        }
    }
}
