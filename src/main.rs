use clap::Parser;
use cli::tool::{Cli, CliCommands};
use graphql_client::GraphQLQuery;

#[allow(clippy::upper_case_acronyms)]
type UUID = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "graphql/tasks.graphql",
    response_derives = "Debug"
)]
pub struct Tasks;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let args = Cli::parse();

    match args.command {
        CliCommands::Get { resource } => {
            println!("Get: {:?}", resource)
        }
        CliCommands::Create => {}
        CliCommands::Set => {}
        CliCommands::Delete => {}
        CliCommands::Login => {}
    }

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }

    // let request_body = Tasks::build_query(tasks::Variables {});

    // let client = reqwest::Client::new();
    // let res = client
    //     .post("/graphql")
    //     .json(&request_body)
    //     .send()
    //     .await
    //     .unwrap();

    // let response_body: Response<tasks::ResponseData> = res.json().await.unwrap();

    // let first_title = response_body
    //     .data
    //     .unwrap()
    //     .tasks
    //     .first()
    //     .unwrap()
    //     .title
    //     .clone();

    // println!("{:#?}", first_title);
}
