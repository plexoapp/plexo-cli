use clap::Parser;
use graphql_client::GraphQLQuery;
// use inquire::{InquireError, Select};
use plexo_cli::tool::{Cli, CliCommands, CommandsGPT, ResourceCreate};

use dotenv::dotenv;

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
    dotenv().ok();

    let args = Cli::parse();

    match args.command {
        CliCommands::Get {
            resource,
            // interactive: _,
        } => {
            println!("Get: {:?}", resource);
        }
        CliCommands::Create { resource } => {
            println!("Create: {:?}", resource);

            match resource {
                // ResourceCreate::Task(task) => {
                //     if task.interactive {
                //         let projects_names: Vec<&str> =
                //             vec!["Plexo Core", "Plexo CLI", "Plexo Platform"];

                //         let ans: Result<&str, InquireError> =
                //             Select::new("Select parent project", projects_names).prompt();

                //         match ans {
                //             Ok(choice) => println!("{} is your selected project", choice),
                //             Err(_) => println!("There was an error, please try again"),
                //         }
                //     }
                //     println!("Task: {:?}", task)
                // }
                ResourceCreate::Projects => {}
                ResourceCreate::Teams => {}
                ResourceCreate::Members => {}
                ResourceCreate::Labels => {}
            }
        }
        // CliCommands::Set => {}
        // CliCommands::Delete => {}
        // CliCommands::Login => {}
        CliCommands::Do {
            // file: _,
            prompt,
        } => {
            println!("Do: {:?}", prompt);

            let max_response_tokens = 1024_u16;
            let request_token_limit = 4191;
            let model_name = "gpt-3.5-turbo";
            let system_message = "You are a helpful function-calling bot. If the user prompt \
                                  involves multiple steps, use the CallMultiStep function with \
                                  a new-line-separated string with each line descripbing a step.";

            let a = CommandsGPT::run(
                &prompt,
                model_name,
                request_token_limit,
                max_response_tokens,
                Some(system_message.to_string()),
                // None,
            )
            .await;

            println!("a: {:?}", a);
        }
        CliCommands::CallMultiStep { prompt_list } => {
            todo!("CallMultiStep: {:?}", prompt_list)
        }
    }
}
