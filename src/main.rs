use chrono::Local;
use std::{fs::OpenOptions, io::Write, sync::Arc};
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide_core::types::Message;
use tokio;

const STATUS_MAX_LENGTH: usize = 50;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a status update.")]
    Status(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command, target_user_id: i64) -> ResponseResult<()> {

    if let Some(user) = msg.from() {
        log::info!("Received a message from user id: {}, username: {:?}", user.id.0, user.username);
    }

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Status(status) => {
            if let Some(user) = msg.from() {
                // how tf is this deprecated
                if user.id.0 == target_user_id as u64 {
                    let username = user.username.as_deref();
                    log_message(&status, username).expect("Failed to log message from @{target_user_id}");
                }
            }

            bot.send_message(msg.chat.id, format!("Status updated: {status}."))
                .await?
        }
    };
    Ok(())
}

fn load_rc() -> std::io::Result<i64> {
    let home_dir = dirs::home_dir().expect("Yuor homeless");
    let config_path = home_dir.join(".klandestinrc");
    let content = std::fs::read_to_string(config_path)?;
    let id: i64 = content
        .trim()
        .parse()
        .expect("Invalid user id format in .klandestinrc");
    Ok(id)
}

fn log_message(message: &str, username: Option<&str>) -> std::io::Result<()> {
    let home_dir = dirs::home_dir().expect("Yuor homeless");
    let log_path = home_dir.join("klandestin_log");
    let status_path = home_dir.join(".klandestin_current");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S");

    match username {
        Some(name) => writeln!(file, "[{timestamp}] @{name}:\n{message}")?,
        None => writeln!(file, "[{timestamp}]\n{message}")?,
    }

    let short_message = prepare_status_text(message);
    {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(status_path)?;

        writeln!(file, "{short_message}")?;
    }

    Ok(())
}

fn prepare_status_text(message: &str) -> String {
    let first_line = message.lines().next().unwrap_or("");
    let mut full_text = format!("{}", first_line);

    if full_text.len() > STATUS_MAX_LENGTH {
        full_text.truncate(STATUS_MAX_LENGTH);
        full_text.push_str("...");
    }

    full_text
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    let target_user_id = load_rc().expect("Failed to load ~/.klandestinrc");
    let shared_target_id = Arc::new(target_user_id);

    // Command::repl(bot, move |bot, msg, cmd| {
    //     let target_user_id: Arc<i64> = Arc::clone(&shared_target_id);
    //     async move { answer(bot, msg, cmd, *target_user_id).await }
    // }).await;

    // Command::repl(bot, answer).await;
    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let target_user_id: Arc<i64> = Arc::clone(&shared_target_id);
        async move { answer(bot, msg, cmd, *target_user_id).await }
    })
    .await;
}
