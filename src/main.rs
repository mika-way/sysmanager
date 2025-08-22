use clap::{Parser, Subcommand};

mod sys_update;
mod sys_clear;
mod sys_info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Befehle für Systemoperationen
    Sys {
        #[command(subcommand)]
        command: SysCommands,
    },
}

#[derive(Subcommand)]
enum SysCommands {
    /// Aktualisiert das System
    Update(UpdateArgs),
    /// Löscht temporäre Dateien
    Clear(ClearArgs),
    /// Infos über dein System
    Info,
}
#[derive(Parser)]
struct UpdateArgs {
    ///Überspringen der Kontrolle
    #[arg(long, default_value_t = true, action = clap::ArgAction::SetFalse)]
    noconfirm: bool,
}
#[derive(Parser)]
struct ClearArgs {
    ///Überspringen der Kontrolle
    #[arg(long, default_value_t = true, action = clap::ArgAction::SetFalse)]
    noconfirm: bool,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Sys { command } => {
            match command {
                SysCommands::Update(args) => {
                    if args.noconfirm == true {
                        sys_update::update(true);
                    }else{
                        sys_update::update(false);
                    }
                }
                SysCommands::Clear(args) => {
                    if args.noconfirm == true {
                        sys_clear::clear(true);
                    }else{
                        sys_clear::clear(false);
                    }
                }
                SysCommands::Info => {
                    sys_info::info();
                }
            }
        }
    }
}
