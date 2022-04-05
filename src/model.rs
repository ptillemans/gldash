use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct Group {
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Clone, Debug)]
pub struct Project {
    pub name: String,
    pub pipelines: Vec<Pipeline>,
}

#[derive(Clone, Debug)]
pub struct Pipeline {
    pub id: String,
    pub status: Status,
    pub jobs: Vec<Job>,
}

#[derive(Clone, Debug)]
pub struct Job {
    pub name: String,
    pub status: Status,
}

#[derive(Clone, Debug)]
pub enum Status {
    PENDING,
    RUNNING,
    OK,
    FAILED,
    BLOCKED,
    SKIPPED,
    MANUAL,
    UNKNOWN,
    CREATED,
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "pending" => Status::PENDING,
            "running" => Status::RUNNING,
            "passed" => Status::OK,
            "failed" => Status::FAILED,
            "blocked" => Status::BLOCKED,
            "skipped" => Status::SKIPPED,
            "manual" => Status::MANUAL,
            "created" => Status::CREATED,
            _ => {
                log::error!("Unknown status found : {}", s);
                Status::UNKNOWN
            }
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::PENDING => write!(f, "PENDING"),
            Status::RUNNING => write!(f, "RUNNING"),
            Status::OK => write!(f, "OK"),
            Status::FAILED => write!(f, "FAILED"),
            Status::BLOCKED => write!(f, "BLOCKED"),
            Status::SKIPPED => write!(f, "SKIPPED"),
            Status::MANUAL => write!(f, "MANUAL"),
            Status::CREATED => write!(f, "CREATED"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

pub struct Configuration {
    pub gitlab_api_token: String,
    pub group_names: Vec<String>,
}
