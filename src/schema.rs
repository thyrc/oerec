use postgres::types::{FromSql, ToSql};
use serde_derive::Serialize;
use std::net::IpAddr;

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "usertype")]
pub enum Usertype {
    #[postgres(name = "AD user")]
    AD,
    #[postgres(name = "tool user")]
    Tool,
    #[postgres(name = "external user")]
    External,
}

#[derive(Debug, Serialize)]
pub struct UserQuery {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub usertype: String,
    pub disabled: String,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SshKeysQuery {
    pub id: i64,
    pub email: String,
    pub sshkey: String,
    pub fingerprint: String,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ServerQuery {
    pub id: i64,
    pub name: String,
    pub ip: IpAddr,
    pub disabled: String,
    pub use_dns: String,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ServerGroupQuery {
    pub servergroup: String,
    pub member: Option<String>,
    pub ip: Option<IpAddr>,
    pub comment: Option<String>,
    pub subgroups: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserGroupQuery {
    pub usergroup: String,
    pub member: Option<String>,
    pub comment: Option<String>,
    pub subgroups: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserAccessQuery {
    pub email: String,
    pub sshuser: String,
    pub serveraccess: String,
    pub ip: Option<IpAddr>,
    pub servername: Option<String>,
    pub usergroup: Option<String>,
    pub servergroup: Option<String>,
    pub until: String,
}

#[derive(Debug, Serialize)]
pub struct ServerAccessQuery {
    pub name: String,
    pub sshuser: String,
    pub server: Option<String>,
    pub ip: Option<IpAddr>,
    pub sshfrom: Option<String>,
    pub sshcommand: Option<String>,
    pub sshoption: Option<String>,
    pub servergroup: Option<String>,
}
