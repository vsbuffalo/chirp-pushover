use clap::{Parser, Subcommand, ValueEnum};
use pushover::{requests::message::SendMessage, API};
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Config {
        #[arg(long)]
        api_token: String,
        #[arg(long)]
        user_key: String,
        #[arg(long)]
        no_wrapper: bool,
    },
    Success {
        #[arg(short, long)]
        message: String,
        #[arg(value_enum, short, long, default_value_t = Priority::Normal)]
        priority: Priority,
    },
    Failure {
        #[arg(short, long)]
        message: String,
        #[arg(value_enum, short, long, default_value_t = Priority::High)]
        priority: Priority,
    },
    Msg {
        #[arg(short, long)]
        message: String,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(value_enum, short, long, default_value_t = Priority::Normal)]
        priority: Priority,
    },
}

#[derive(Clone, ValueEnum)]
enum Priority {
    Lowest,
    Low,
    Normal,
    High,
    Emergency,
}

#[derive(Serialize, Deserialize)]
struct Config {
    api_token: String,
    user_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Config {
            api_token,
            user_key,
            no_wrapper,
        } => {
            save_config(api_token, user_key)?;
            println!("Configuration saved successfully.");
            if !no_wrapper {
                install_wrapper()?;
            }
        }
        Commands::Success { message, priority } => {
            send_notification("Success Notification", message, priority)?;
        }
        Commands::Failure { message, priority } => {
            send_notification("Failure Notification", message, priority)?;
        }
        Commands::Msg {
            message,
            title,
            priority,
        } => {
            let title = title.as_deref().unwrap_or("Custom Notification");
            send_notification(title, message, priority)?;
        }
    }
    Ok(())
}

fn save_config(api_token: &str, user_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        api_token: api_token.to_string(),
        user_key: user_key.to_string(),
    };
    let config_path = get_config_path();
    let config_str = serde_yaml::to_string(&config)?;
    fs::write(config_path, config_str)?;
    Ok(())
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_str)?;
    Ok(config)
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Unable to find home directory");
    path.push(".pushover_tokens.yml");
    path
}

fn send_notification(
    title: &str,
    message: &str,
    priority: &Priority,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let api = API::new();
    let mut msg = SendMessage::new(&config.api_token, &config.user_key, message);
    msg.set_title(title);
    let pushover_priority = match priority {
        Priority::Lowest => pushover::Priority::Lowest,
        Priority::Low => pushover::Priority::Low,
        Priority::Normal => pushover::Priority::Normal,
        Priority::High => pushover::Priority::High,
        Priority::Emergency => pushover::Priority::Emergency {
            retry: 30,
            expire: 10800,
            callback_url: None,
        },
    };
    msg.set_priority(pushover_priority);
    api.send(&msg)?;
    println!("Notification sent successfully.");
    Ok(())
}

fn install_wrapper() -> Result<(), Box<dyn std::error::Error>> {
    let wrapper_script = include_str!("../scripts/chirper.sh");

    let mut path = dirs::home_dir().expect("Unable to find home directory");
    path.push(".local/bin");

    // Create .local/bin if it doesn't exist
    fs::create_dir_all(&path)?;

    path.push("chirper");
    fs::write(&path, wrapper_script)?;

    // Make the script executable
    let mut perms = fs::metadata(&path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms)?;

    println!(
        "Wrapper script 'chirper' installed successfully to {}.",
        path.display()
    );
    println!("Please ensure that ~/.local/bin is in your PATH.");
    println!("You can add it by adding the following line to your ~/.bashrc or ~/.zshrc:");
    println!("export PATH=\"$HOME/.local/bin:$PATH\"");

    Ok(())
}

