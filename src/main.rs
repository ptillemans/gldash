use anyhow::*;
use clap::Parser;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use log::*;
use prettytable::*;
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

#[derive(Parser)]
#[clap(author, about, version)]
struct Command {
    #[clap(name = "group")]
    group: String,
}

fn format_pipeline_results(
    pipelines: &pipelines_query::PipelinesQueryGroupProjectsNodesPipelines,
) -> String {
    pipelines
        .nodes
        .as_ref()
        .into_iter()
        .flatten()
        .map(|result| match result {
            Some(pl) => pl.detailed_status.text.as_ref().unwrap().to_string(),
            None => "---".to_string(),
        })
        .next()
        .unwrap_or("???".to_string())
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let gitlab_api_token = std::env::var("GRAPHQL_TOKEN").expect("Missing GRAPHQL_TOKEN env var");

    let args = Command::parse();

    let group = args.group;
    let variables = pipelines_query::Variables { group: group };

    let client = Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", gitlab_api_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .user_agent("graphql-rust/0.10.0")
        .build()?;

    let response_body = post_graphql::<PipelinesQuery, _>(
        &client,
        "https://gitlab.melexis.com/api/graphql",
        variables,
    )
    .unwrap();

    info!("{:?}", response_body);

    let response_data: pipelines_query::ResponseData =
        response_body.data.expect("missing response data");

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "project", "result"));

    for node in response_data
        .group
        .expect("missing group")
        .projects
        .nodes
        .expect("no projects")
        .iter()
    {
        if let Some(project) = node {
            let jobs = if let Some(pipelines) = &project.pipelines {
                format_pipeline_results(pipelines)
            } else {
                "???".to_string()
            };
            table.add_row(row!(project.name, jobs));
        }
    }

    table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repo_name_works() {
        assert_eq!(
            parse_repo_name("graphql-rust/graphql-client").unwrap(),
            ("graphql-rust", "graphql-client")
        );
        assert!(parse_repo_name("abcd").is_err());
    }
}
