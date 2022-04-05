use anyhow::*;
use clap::Parser;
use gldash::model::Configuration;
use gldash::query::fetch_job_info;
use gldash::ui::print_groups;

#[derive(Parser)]
#[clap(author, about, version)]
struct Command {
    #[clap(name = "group")]
    group_names: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let gitlab_api_token = std::env::var("GRAPHQL_TOKEN").expect("Missing GRAPHQL_TOKEN env var");

    let args = Command::parse();

    let group_names = args.group_names;

    let config = Configuration {
        gitlab_api_token,
        group_names,
    };

    let groups = fetch_job_info(config)?;

    print_groups(groups);

    Ok(())
}
