use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true, disable_version_flag = true)]
pub struct App {
    /// Run the given command in background mode. Caution: backgrounded processes are not subject to shell job control. Interactive commands may misbehave.
    #[arg(short, long)]
    pub background: bool,
    /// run as the specified user
    #[arg(short, long)]
    pub user: Option<String>,
    #[arg(short, long)]
    pub validate: bool,
    pub cmd: Option<Vec<String>>,
    // /// Run the command from the specified directory. The security policy may return an error if the user does not have permission to specify the working directory.
    // #[arg(short = 'D', long)]
    // pub chdir: Option<PathBuf>,
    // /// Indicates to the security policy that the user wishes to preserve existing environment variables. Subject to security policy.
    // #[arg(short, long)]
    // pub preserve_env: bool,
    // #[arg(long)]
    // help: bool,
    // /// Specify group via name or `#<group number>`
    // #[arg(short, long)]
    // pub group: Option<String>,
    // #[arg(short = 'H', long)]
    // pub set_home: bool,
    // #[arg(short = 'h', long)]
    // pub host: Option<String>,
    // #[arg(short = 'i', long)]
    // pub login: bool,
    // /// Remove all cached credentials for user.
    // #[arg(short = 'K', long)]
    // pub remove_timestamp: bool,
    // /// Remove current shell's cached credentials.
    // #[arg(short, long)]
    // pub reset_timestamp: bool,
    // /// Don't update cached credentials.
    // #[arg(short = 'N', long)]
    // pub no_update: bool,
    // /// List privileges for user.
    // #[arg(short, long)]
    // pub list: Option<String>,
    // #[arg(short = 'R', long)]
    // pub chroot: Option<PathBuf>,
    // /// Write prompt to stderr and read from stdin instead of terminal input.
    // #[arg(short = 'S', long)]
    // pub stdin: bool,
    // /// Run shell specified in `SHELL`.
    // #[arg(short, long)]
    // pub shell: bool,
}
