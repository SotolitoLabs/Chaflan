// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::*;
use rocket::serde::{ Deserialize, Serialize };

use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::prelude::*;
#[derive(Clone, Debug, Identifiable, Queryable, QueryableByName, Insertable, Serialize, Deserialize)]
#[diesel(table_name = events)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub venue: String,
    pub address: Option<String>,
    pub image: Option<String>,
    pub comments: Option<String>,
    pub contactname: Option<String>,
    pub starts_at: NaiveDateTime,
    pub ends_at: NaiveDateTime,
}

#[derive(Clone, Debug, Identifiable, Queryable, QueryableByName, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: String,
    pub password: Option<String>,
    pub oauth_provider: String,
    pub oauth_user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

