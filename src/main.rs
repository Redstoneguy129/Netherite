use clap::{Parser,Subcommand};
use clap::builder::ArgAction;

#[derive(Parser)]
#[command(
    name = "Netherite",
    version = "0.1.0",
    author = "Redstoneguy129",
    about = "Minecraft Launcher CLI",
    long_about = "Netherite is a Minecraft Launcher CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug,Subcommand)]
enum Commands {
    #[command(about = "Run Minecraft")]
    Run {
        profile: String,
        account: String,
    },
    #[command(about = "Install a Minecraft version")]
    Install {
        version: Option<String>,
    },
    #[command(about = "Uninstall a Minecraft version", arg_required_else_help = true)]
    Remove {
        version: String,
    },
    #[command(about = "List installed Minecraft version(s)")]
    List {
        #[clap(long, action = ArgAction::SetTrue)]
        snapshots: bool,
    },
    #[command(subcommand)]
    Account(Account),
    #[command(subcommand)]
    Profile(Profile),
}

#[derive(Debug,Subcommand)]
#[command(about = "Manage Minecraft account(s)")]
enum Account {
    #[command(about = "Login to a Minecraft account")]
    Login {
        username: String,
        password: String,
        #[clap(long, value_name = "Auth Server")]
        auth: Option<String>,
    },
    #[command(about = "Logout of a Minecraft account")]
    Logout {
        username: String,
    },
    #[command(about = "List Minecraft account(s)")]
    List,
}

#[derive(Debug,Subcommand)]
#[command(about = "Manage Minecraft profile(s)")]
enum Profile {
    #[command(about = "Create a new Minecraft profile")]
    Create {
        name: String,
        version: String,
        #[clap(long, value_name = "Game Directory")]
        directory: Option<String>,
        #[clap(long, action = ArgAction::SetTrue)]
        offline: bool,
    },
    #[command(about = "Delete a Minecraft profile")]
    Delete {
        name: String,
        #[clap(long, action = ArgAction::SetTrue)]
        force: bool,
    },
    #[command(about = "List Minecraft profile(s)")]
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { profile, account } => {
            println!("Running Minecraft profile {} with account {}", profile, account);
        }
        Commands::Install { version } => {
            let version = version.as_deref().unwrap_or("latest");
            println!("Installing Minecraft version {}", version);
        }
        Commands::Remove { version } => {
            println!("Removing Minecraft version {}", version);
        }
        Commands::List { snapshots } => {
            if snapshots {
                println!("Listing installed Minecraft snapshots");
            } else {
                println!("Listing installed Minecraft releases");
            }
        }
        Commands::Account(account) => {
            match account {
                Account::Login { username, password, auth } => {
                    if auth.clone().is_none() {
                        println!("Logging into Minecraft account {} with password {}", username, password);
                    } else {
                        println!("Logging into Minecraft account {} with password {} and auth server {}", username, password, auth.unwrap());
                    }
                }
                Account::Logout { username } => {
                    println!("Logging out of Minecraft account {}", username);
                }
                Account::List => {
                    println!("Listing Minecraft accounts");
                }
            }
        }
        Commands::Profile(profile) => {
            match profile {
                Profile::Create { name, version, directory, offline: _ } => {
                    if directory.clone().is_none() {
                        println!("Creating Minecraft profile {} with version {}", name, version);
                    } else {
                        println!("Creating Minecraft profile {} with version {} and game directory {}", name, version, directory.unwrap());
                    }
                }
                Profile::Delete { name, force } => {
                    if force {
                        println!("Deleting Minecraft profile {} and all associated files", name);
                    } else {
                        println!("Deleting Minecraft profile {}", name);
                    }
                }
                Profile::List => {
                    println!("Listing Minecraft profiles");
                }
            }
        }
    }
}