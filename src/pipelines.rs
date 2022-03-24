pub struct PipelinesQuery;
pub mod pipelines_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PipelinesQuery";
    pub const QUERY : & str = "query PipelinesQuery($group: ID!) {\n  group(fullPath: $group) {\n    id\n    projects {\n      nodes {\n        id\n        name\n        pipelines(first: 1) {\n          nodes {\n            id\n            finishedAt\n            detailedStatus {\n              text\n            }\n            stages(first: 10) {\n              nodes {\n                name\n                detailedStatus {\n                  detailsPath\n                  favicon\n                  group\n                  hasDetails\n                  icon\n                  label\n                  text\n                  tooltip\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type Time = super::Time;
    #[derive(Serialize)]
    pub struct Variables {
        pub group: ID,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub group: Option<PipelinesQueryGroup>,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroup {
        pub id: ID,
        pub projects: PipelinesQueryGroupProjects,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroupProjects {
        pub nodes: Option<Vec<Option<PipelinesQueryGroupProjectsNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroupProjectsNodes {
        pub id: ID,
        pub name: String,
        pub pipelines: Option<PipelinesQueryGroupProjectsNodesPipelines>,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroupProjectsNodesPipelines {
        pub nodes: Option<Vec<Option<PipelinesQueryGroupProjectsNodesPipelinesNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroupProjectsNodesPipelinesNodes {
        pub id: ID,
        #[serde(rename = "finishedAt")]
        pub finished_at: Option<Time>,
        #[serde(rename = "detailedStatus")]
        pub detailed_status: PipelinesQueryGroupProjectsNodesPipelinesNodesDetailedStatus,
        pub stages: Option<PipelinesQueryGroupProjectsNodesPipelinesNodesStages>,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroupProjectsNodesPipelinesNodesDetailedStatus {
        pub text: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroupProjectsNodesPipelinesNodesStages {
        pub nodes: Option<Vec<Option<PipelinesQueryGroupProjectsNodesPipelinesNodesStagesNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroupProjectsNodesPipelinesNodesStagesNodes {
        pub name: Option<String>,
        #[serde(rename = "detailedStatus")]
        pub detailed_status:
            Option<PipelinesQueryGroupProjectsNodesPipelinesNodesStagesNodesDetailedStatus>,
    }
    #[derive(Deserialize)]
    pub struct PipelinesQueryGroupProjectsNodesPipelinesNodesStagesNodesDetailedStatus {
        #[serde(rename = "detailsPath")]
        pub details_path: Option<String>,
        pub favicon: Option<String>,
        pub group: Option<String>,
        #[serde(rename = "hasDetails")]
        pub has_details: Option<Boolean>,
        pub icon: Option<String>,
        pub label: Option<String>,
        pub text: Option<String>,
        pub tooltip: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for PipelinesQuery {
    type Variables = pipelines_query::Variables;
    type ResponseData = pipelines_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: pipelines_query::QUERY,
            operation_name: pipelines_query::OPERATION_NAME,
        }
    }
}
