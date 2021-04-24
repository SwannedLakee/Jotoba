use std::{convert::TryFrom, fmt::Display, io::Write};

use crate::error;
use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Integer,
    types::{FromSql, ToSql},
};
use strum_macros::{AsRefStr, EnumString};

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
#[sql_type = "Integer"]
pub enum NameType {
    #[strum(serialize = "company")]
    Company,
    #[strum(serialize = "fem")]
    Female,
    #[strum(serialize = "masc")]
    Male,
    #[strum(serialize = "given")]
    Given,
    #[strum(serialize = "organization")]
    Organization,
    #[strum(serialize = "person")]
    Person,
    #[strum(serialize = "place")]
    Place,
    #[strum(serialize = "product")]
    Product,
    #[strum(serialize = "station")]
    RailwayStation,
    #[strum(serialize = "surname")]
    Surname,
    #[strum(serialize = "unclass")]
    Unclassified,
    #[strum(serialize = "work")]
    Work,
}

impl NameType {
    pub fn humanized(self) -> String {
        match self {
            NameType::Company => "Company",
            NameType::Female => "Female",
            NameType::Male => "Male",
            NameType::Given => "Given name",
            NameType::Organization => "Organization",
            NameType::Person => "Persons name",
            NameType::Place => "Place",
            NameType::Product => "Product",
            NameType::RailwayStation => "(Railway)Station",
            NameType::Surname => "Surname",
            NameType::Unclassified => "Unknown",
            NameType::Work => "Art work",
        }
        .to_string()
    }

    pub fn is_gender(&self) -> bool {
        matches!(self, Self::Female | Self::Male)
    }
}

impl Display for NameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.humanized())
    }
}

impl ToSql<Integer, Pg> for NameType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <i32 as ToSql<Integer, Pg>>::to_sql(&(*self).into(), out)
    }
}

impl FromSql<Integer, Pg> for NameType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::try_from(<i32 as FromSql<Integer, Pg>>::from_sql(
            bytes,
        )?)?)
    }
}

impl TryFrom<i32> for NameType {
    type Error = crate::error::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => NameType::Company,
            1 => NameType::Female,
            2 => NameType::Male,
            3 => NameType::Given,
            4 => NameType::Organization,
            5 => NameType::Person,
            6 => NameType::Place,
            7 => NameType::Product,
            8 => NameType::RailwayStation,
            9 => NameType::Surname,
            10 => NameType::Unclassified,
            11 => NameType::Work,
            _ => return Err(error::Error::ParseError),
        })
    }
}

impl Into<i32> for NameType {
    fn into(self) -> i32 {
        match self {
            NameType::Company => 0,
            NameType::Female => 1,
            NameType::Male => 2,
            NameType::Given => 3,
            NameType::Organization => 4,
            NameType::Person => 5,
            NameType::Place => 6,
            NameType::Product => 7,
            NameType::RailwayStation => 8,
            NameType::Surname => 9,
            NameType::Unclassified => 10,
            NameType::Work => 11,
        }
    }
}