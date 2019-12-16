use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub type RunId = i32;
pub type InvocationId = i32;
pub type UserId = uuid::Uuid;
pub type ProblemId = String;
pub type ContestId = String;
pub type RegistrationId = i32;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, PartialEq, Eq)]
pub struct Run {
    pub id: RunId,
    pub toolchain_id: String,
    pub status_code: String,
    pub status_kind: String,
    pub problem_id: ProblemId,
    pub score: i32,
    pub rejudge_id: i32,
    pub user_id: UserId,
}

#[derive(Insertable)]
#[table_name = "runs"]
pub struct NewRun {
    pub toolchain_id: String,
    pub status_code: String,
    pub status_kind: String,
    pub problem_id: ProblemId,
    pub score: i32,
    pub rejudge_id: i32,
    pub user_id: UserId,
}

#[derive(AsChangeset, Default)]
#[table_name = "runs"]
pub struct RunPatch {
    pub status_code: Option<String>,
    pub status_kind: Option<String>,
    #[column_name = "score"]
    pub score: Option<i32>,
    #[column_name = "rejudge_id"]
    pub rejudge_id: Option<i32>,
}

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RawInvocation {
    pub(crate) id: InvocationId,
    pub(crate) invoke_task: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "invocations"]
pub(crate) struct RawNewInvocation {
    pub(crate) invoke_task: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Invocation {
    pub id: InvocationId,
    pub invoke_task: invoker_api::InvokeTask,
}

impl Invocation {
    pub(crate) fn from_raw(raw: &RawInvocation) -> Result<Self> {
        Ok(Self {
            id: raw.id,
            invoke_task: bincode::deserialize(&raw.invoke_task)
                .context("failed to deserialize InvokeTask")?,
        })
    }
}

impl NewInvocation {
    pub(crate) fn to_raw(&self) -> Result<RawNewInvocation> {
        Ok(RawNewInvocation {
            invoke_task: bincode::serialize(&self.invoke_task)
                .context("failed to serialize InvokeTask")?,
        })
    }
}

pub struct NewInvocation {
    pub invoke_task: invoker_api::InvokeTask,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password_hash: Option<String>,
    pub groups: Vec<String>,
}

pub struct NewUser {
    pub username: String,
    pub password_hash: Option<String>,
    pub groups: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct Registration {
    pub id: RegistrationId,
    pub user_id: UserId,
    pub contest_id: ContestId,
}

#[derive(Insertable)]
#[table_name = "registrations"]
pub struct NewRegistration {
    pub user_id: UserId,
    pub contest_id: ContestId,
}

use diesel::sql_types::*;

include!("./schema_raw.rs");
