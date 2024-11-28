use crate::config::get_config;
use anyhow::Result;
use log::{error, info};
use std::process::Command;

// TODO: Filter out users to revoke to only the ones in affected-users as well
pub fn revoke_admin() -> Result<()> {
    let config = get_config()?;
    let groups_to_remove = vec!["wheel", "sudo"];

    for username in config.revoke_admin {
        for group in &groups_to_remove {
            let result = Command::new("gpasswd")
                .arg("-d")
                .arg(&username)
                .arg(group)
                .output();

            let output = match result {
                Ok(out) => out,
                Err(error) => {
                    error!("Error executing gpasswd: {error}");
                    continue;
                }
            };

            if output.status.success() {
                info!("Successfully removed {username} from {group}");
            } else {
                error!(
                    "Error removing {username} from {group}: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }
    Ok(())
}
