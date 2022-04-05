use crate::model::{Group, Pipeline};
use prettytable::*;

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

pub fn print_groups(groups: Vec<Group>) {
    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "project", "result", "jobs"));

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
}
