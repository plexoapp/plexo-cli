use clap::Parser;
use graphql_client::GraphQLQuery;
use inquire::{InquireError, Select};
use plexo_cli::tool::{Cli, CliCommands, ResourceCreate};

#[allow(clippy::upper_case_acronyms)]
type UUID = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/tasks.graphql",
    response_derives = "Debug"
)]
pub struct Tasks;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        CliCommands::Get {
            resource,
            interactive: _,
        } => {
            println!("Get: {:?}", resource);
        }
        CliCommands::Create { resource } => {
            println!("Create: {:?}", resource);

            match resource {
                ResourceCreate::Task(task) => {
                    if task.interactive {
                        let projects_names: Vec<&str> =
                            vec!["Plexo Core", "Plexo CLI", "Plexo Platform"];

                        let ans: Result<&str, InquireError> =
                            Select::new("Select parent project", projects_names).prompt();

                        match ans {
                            Ok(choice) => println!("{} is your selected project", choice),
                            Err(_) => println!("There was an error, please try again"),
                        }
                    }
                    println!("Task: {:?}", task)
                }
                ResourceCreate::Projects => {}
                ResourceCreate::Teams => {}
                ResourceCreate::Members => {}
                ResourceCreate::Labels => {}
            }
        }
        CliCommands::Set => {}
        CliCommands::Delete => {}
        CliCommands::Login => {}
        CliCommands::Do { file: _, query } => {
            println!("Do: {:?}", query);
        }
    }
}
