use bot::{dler, Args, Commands};
use clap::Parser;
use teloxide::Bot;

fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let token = if let Some(token) = args.token {
        token
    } else {
        return;
    };

    init_logger();
    let bot = Bot::new(token);
    match args.command {
        Commands::Dler {
            email,
            password,
            chat,
        } => {
            dler::run(email, password, &bot, chat).await;
        }
    }
}
