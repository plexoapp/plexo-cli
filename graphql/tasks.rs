#![allow(clippy::all, warnings)]
pub struct Tasks;
pub mod tasks {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Tasks";
    pub const QUERY : & str = "query Tasks {\n  tasks {\n    id\n    title\n  }\n}\n\nquery TasksWithOwner {\n  tasks {\n    id\n    title\n    owner {\n      id\n      name\n    }\n  }\n}\n" ;
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
    type UUID = super::UUID;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub tasks: Vec<TasksTasks>,
    }
    #[derive(Deserialize)]
    pub struct TasksTasks {
        pub id: UUID,
        pub title: String,
    }
}
impl graphql_client::GraphQLQuery for Tasks {
    type Variables = tasks::Variables;
    type ResponseData = tasks::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: tasks::QUERY,
            operation_name: tasks::OPERATION_NAME,
        }
    }
}
pub struct TasksWithOwner;
pub mod tasks_with_owner {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TasksWithOwner";
    pub const QUERY : & str = "query Tasks {\n  tasks {\n    id\n    title\n  }\n}\n\nquery TasksWithOwner {\n  tasks {\n    id\n    title\n    owner {\n      id\n      name\n    }\n  }\n}\n" ;
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
    type UUID = super::UUID;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub tasks: Vec<TasksWithOwnerTasks>,
    }
    #[derive(Deserialize)]
    pub struct TasksWithOwnerTasks {
        pub id: UUID,
        pub title: String,
        pub owner: Option<TasksWithOwnerTasksOwner>,
    }
    #[derive(Deserialize)]
    pub struct TasksWithOwnerTasksOwner {
        pub id: UUID,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for TasksWithOwner {
    type Variables = tasks_with_owner::Variables;
    type ResponseData = tasks_with_owner::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: tasks_with_owner::QUERY,
            operation_name: tasks_with_owner::OPERATION_NAME,
        }
    }
}
