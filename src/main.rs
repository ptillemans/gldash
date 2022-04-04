use anyhow::*;
use clap::Parser;
use gldash::model::{Configuration, Pipeline};
use gldash::query::fetch_job_info;
use prettytable::*;

#[derive(Parser)]
#[clap(author, about, version)]
struct Command {
    #[clap(name = "group")]
    group: String,
}

fn format_pipeline_results(pipelines: &[Pipeline]) -> String {
    pipelines
        .get(0)
        .map(|pipeline| pipeline.jobs.clone())
        .map({
            |jobs| {
                jobs.into_iter()
                    .map(|job| job.status.to_string())
                    .collect::<Vec<String>>()
                    .join(" > ")
            }
        })
        .unwrap_or_else(|| "???".to_string())
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let gitlab_api_token = std::env::var("GRAPHQL_TOKEN").expect("Missing GRAPHQL_TOKEN env var");

    let args = Command::parse();

    let group = args.group;

    let config = Configuration {
        gitlab_api_token,
        group_names: vec![group],
    };

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "project", "result", "jobs"));

    let groups = fetch_job_info(config)?;

    for project in groups.into_iter().flat_map(|g| g.projects) {
        let project_status = project
            .pipelines
            .get(0)
            .map(|pl| pl.status.to_string())
            .unwrap_or_else(|| "???".to_string());
        table.add_row(row!(
            project.name,
            project_status,
            format_pipeline_results(&project.pipelines)
        ));
    }

    table.printstd();
    Ok(())
}
