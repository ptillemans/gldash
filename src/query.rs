use crate::model::{Configuration, Group, Job, Pipeline, Project};
use anyhow::{Context, Result};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use log::info;
use reqwest::blocking::Client;

#[allow(clippy::upper_case_acronyms)]
type Time = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/pipelines-schema.json",
    query_path = "src/pipelines.graphql",
    response_derives = "Debug"
)]
struct PipelinesQuery;

impl TryFrom<pipelines_query::PipelinesQueryGroup> for Group {
    type Error = anyhow::Error;

    fn try_from(value: pipelines_query::PipelinesQueryGroup) -> Result<Self, Self::Error> {
        let name = value.name;
        let projects = value.projects.nodes.expect("missing projects");
        let projects: Vec<Project> = projects
            .iter()
            .flatten()
            .flat_map(Project::try_from)
            .collect();
        Ok(Group { name, projects })
    }
}

impl TryFrom<&pipelines_query::PipelinesQueryGroupProjectsNodes> for Project {
    type Error = anyhow::Error;

    fn try_from(
        value: &pipelines_query::PipelinesQueryGroupProjectsNodes,
    ) -> Result<Self, Self::Error> {
        let pipelines = value.pipelines.as_ref().expect("pipelines expected");
        let pipelines = pipelines.nodes.as_ref().expect("pipeline nodes expected");
        let pipelines = pipelines
            .iter()
            .flatten()
            .flat_map(Pipeline::try_from)
            .collect::<Vec<Pipeline>>();
        Ok(Project {
            name: value.name.to_owned(),
            pipelines,
        })
    }
}

impl TryFrom<&pipelines_query::PipelinesQueryGroupProjectsNodesPipelinesNodes> for Pipeline {
    type Error = anyhow::Error;

    fn try_from(
        value: &pipelines_query::PipelinesQueryGroupProjectsNodesPipelinesNodes,
    ) -> Result<Self, Self::Error> {
        let status = value
            .detailed_status
            .text
            .as_ref()
            .unwrap_or(&"unknown".to_string())
            .as_str()
            .into();
        let jobs = value
            .stages
            .as_ref()
            .expect("some stages present")
            .nodes
            .as_ref()
            .expect("some stage nodes present")
            .iter()
            .flatten()
            .flat_map(Job::try_from)
            .collect::<Vec<Job>>();

        Ok(Pipeline {
            id: value.id.to_owned(),
            status,
            jobs,
        })
    }
}

impl TryFrom<&pipelines_query::PipelinesQueryGroupProjectsNodesPipelinesNodesStagesNodes> for Job {
    type Error = anyhow::Error;

    fn try_from(
        value: &pipelines_query::PipelinesQueryGroupProjectsNodesPipelinesNodesStagesNodes,
    ) -> Result<Self, Self::Error> {
        let status = value
            .detailed_status
            .as_ref()
            .map(|ds| ds.text.as_ref().expect("status text expected"))
            .unwrap_or(&"unknown".to_string())
            .as_str()
            .into();
        let name = value
            .name
            .as_ref()
            .expect("stage to have a name")
            .to_owned();

        Ok(Job { name, status })
    }
}

pub fn fetch_job_info(config: Configuration) -> Result<Vec<Group>> {
    let client = create_http_client(&config)?;

    let names = config.group_names;
    names
        .into_iter()
        .map(|group| query_group(&client, group))
        .collect()
}

fn create_http_client(config: &Configuration) -> Result<Client> {
    Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!(
                    "Bearer {}",
                    config.gitlab_api_token
                ))
                .unwrap(),
            ))
            .collect(),
        )
        .user_agent("graphql-rust/0.10.0")
        .build()
        .context("Unable to build reqwest client")
}

fn query_group(client: &reqwest::blocking::Client, group: String) -> Result<Group> {
    let variables = pipelines_query::Variables { group };
    let response = post_graphql::<PipelinesQuery, _>(
        client,
        "https://gitlab.melexis.com/api/graphql",
        variables,
    )
    .expect("missing response");

    info!("{:?}", response);

    let response_data: pipelines_query::ResponseData = response.data.unwrap();

    response_data.group.map(Group::try_from).unwrap()
}
