use clap::Parser;

pub mod dler;

#[derive(Parser)]
#[command(disable_colored_help = true, disable_help_subcommand = true)]
pub struct Args {
    /// Bot Token
    #[arg(short, long, value_name = "TOKEN", global = true)]
    pub token: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Dler Cloud
    Dler {
        /// Email
        #[arg(long, value_name = "EMAIL")]
        email: String,
        /// Password
        #[arg(long, value_name = "PASSWORD")]
        password: String,
        /// Chat ID
        #[arg(long, value_name = "ID")]
        chat: i64,
    },
}
