use apply::{ApplyArgs, ApplyCommands, ApplySystemCommands};
use clap::Subcommand;

pub mod apply;
pub mod edit;
pub mod get_config;

#[derive(Debug, Subcommand)]
pub enum Commands {
    // Apply changes to the system based on the configuration files
    Apply(ApplyArgs),

    // Get the config just as the code has it, all parsed and everything
    GetConfig,

    // Allow approved config file patches without the need for admin privileges
    Edit { jq_evaluation: String },
}

pub fn run_command(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::GetConfig => get_config::print_config(),
        Commands::Apply(args) => match args.command {
            ApplyCommands::System(args) => {
                if let Err(error) = karen::escalate_if_needed() {
                    return Err(anyhow::anyhow!("Error escalating privileges: {error}"));
                }

                match args.command {
                    ApplySystemCommands::All => apply::all::apply_all(),
                    ApplySystemCommands::DelayedEdits => todo!(),
                    ApplySystemCommands::RevokeAdmin => apply::revoke_admin::apply_revoke_admin(),
                    ApplySystemCommands::BlockNetworking => {
                        apply::block_networking::apply_block_networking()
                    }
                    ApplySystemCommands::BlockFlatpaks => {
                        apply::block_flatpaks::apply_block_flatpaks()
                    }
                }
            }
        },
        Commands::Edit { jq_evaluation } => edit::edit(jq_evaluation),
    }
}
