use clap::{Parser,Subcommand};

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
    #[command(about = "Install a Minecraft version")]
    Install {
        version: Option<String>,
    },
    #[command(about = "Uninstall a Minecraft version", arg_required_else_help = true)]
    Remove {
        version: String,
    },
    #[command(about = "List installed Minecraft versions")]
    List {
        #[clap(long)]
        snapshots: Option<bool>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { version } => {
            let version = version.as_deref().unwrap_or("latest");
            println!("Installing Minecraft version {}", version);
        }
        Commands::Remove { version } => {
            println!("Removing Minecraft version {}", version);
        }
        Commands::List { snapshots } => {
            let snapshots = snapshots.unwrap_or(false);
            if snapshots {
                println!("Listing all Minecraft snapshots");
            } else {
                println!("Listing all Minecraft releases");
            }
        }
    }
}