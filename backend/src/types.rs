use chrono::{DateTime, Utc};
use rusqlite::{types::FromSql, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: UserId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserGroupsResponse {
    pub groups: Vec<UserGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGroup {
    pub group_id: GroupId,
    pub joined: DateTime<Utc>,
    pub commitment: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInboxResponse {
    pub entries: Vec<UserInboxEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupResponse {
    pub name: String,
    pub url: Option<String>,
    pub email: Option<String>,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub profile_photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInboxEntry {
    pub group: GroupId,
    pub title: String,
    pub body: String,
    pub viewed: bool,
    pub announcement: AnnouncementId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupSearchOptions {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupSearchResponse {
    pub entries: Vec<GroupSearchEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupSearchEntry {
    pub id: GroupId,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(u32);

impl FromSql for UserId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        u32::column_result(value).map(Self)
    }
}

impl ToSql for UserId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GroupId(u32);

impl FromSql for GroupId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        u32::column_result(value).map(Self)
    }
}

impl ToSql for GroupId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnnouncementId(u32);

impl FromSql for AnnouncementId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        u32::column_result(value).map(Self)
    }
}

impl ToSql for AnnouncementId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}
