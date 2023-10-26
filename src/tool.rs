// use std::path::PathBuf;

use async_trait::async_trait;
use clap::{command, Parser, Subcommand};
// use openai_func_enums::{generate_enum_info, generate_value_arg_info, RunCommand, SubcommandGPT};
// use crate::commands::tasks::{TaskCreate, TaskGet};
use async_openai::{
    types::{
        ChatCompletionFunctionCall, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, FunctionCall, Role,
    },
    Client,
};
use openai_func_enums::{
    generate_enum_info,
    generate_value_arg_info, // arg_description,
    get_function_chat_completion_args,
    CommandError,
    EnumDescriptor,
    RunCommand,
    SubcommandGPT,
    VariantDescriptors,
};

use serde::Deserialize;
use serde_json::{json, Value};
use tiktoken_rs::cl100k_base;

/// Plexo CLI tool.
#[derive(Parser)]
#[command(name = "plexo-cli")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Debug, Subcommand, SubcommandGPT)]
pub enum CliCommands {
    // /// Login to your Plexo instance.
    // Login,
    /// List one or more resources.
    Get {
        #[command(subcommand)]
        resource: ResourceGet,
        // #[arg(long, short)]
        // interactive: bool,
    },
    /// Create one or more resources from a file or stdin.
    Create {
        #[command(subcommand)]
        resource: ResourceCreate,
        // #[arg(long, short)]
        // interactive: bool,
    },
    // /// Set resources either from a file, stdin, or specifying label selectors, names, resource selectors, or resources.
    // Set,
    // /// Delete resources either from a file, stdin, or specifying label selectors, names, resource selectors, or resources.
    // Delete,
    /// Process a natural language query and execute it.
    Do {
        // #[arg(short, long)]
        // file: Option<PathBuf>,

        // #[arg(short, long)]
        prompt: String,
    },

    /// A string with plain english prompts on separate lines describing the steps to accomplish a multistep request.
    CallMultiStep { prompt_list: String },
    // GPT { prompt: String },
}

#[derive(
    Subcommand, Clone, Copy, Debug, PartialEq, Deserialize, EnumDescriptor, VariantDescriptors,
)]
pub enum ResourceGet {
    // Tasks(TaskGet),
    Projects,
    Teams,
    Members,
    Labels,
}

#[derive(Clone, Debug, PartialEq, Subcommand, Deserialize, EnumDescriptor, VariantDescriptors)]
pub enum ResourceCreate {
    // #[command(name = "tasks")]
    // Task(TaskCreate),
    Projects,
    Teams,
    Members,
    Labels,
}

#[async_trait]
impl RunCommand for CliCommands {
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn run(
        &self,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        println!("Running command: {:?}", self);

        Ok(None)
    }
}
