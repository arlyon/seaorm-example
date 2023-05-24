// main.rs

mod entities;

use std::process::exit;

use clap::{Parser, Subcommand};
use entities::{prelude::*, *};
use sea_orm::{ActiveValue, Database, DbErr, EntityTrait};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "mysql://root:my-secret-pw@localhost:3306/mysql";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// add a chef
    Chef { bakery: i32, name: String },
    /// add a bakery
    Bakery { name: String, profit_margin: f64 },
    /// list all chefs and bakeries
    List,
}

async fn run() -> Result<(), DbErr> {
    let args = Cli::parse();
    let db = Database::connect(DATABASE_URL).await?;

    match args.command {
        Commands::Bakery {
            name,
            profit_margin,
        } => {
            let bakery = bakery::ActiveModel {
                name: ActiveValue::Set(name),
                profit_margin: ActiveValue::Set(profit_margin),
                ..Default::default()
            };
            Bakery::insert(bakery).exec(&db).await?;
        }
        Commands::Chef { bakery, name } => {
            // Finding by id is built-in
            let bakery = match Bakery::find_by_id(bakery).one(&db).await? {
                Some(b) => b,
                None => {
                    println!("bakery not found");
                    exit(1);
                }
            };

            let chef = chef::ActiveModel {
                name: ActiveValue::Set(name),
                bakery_id: ActiveValue::Set(bakery.id),
                ..Default::default()
            };
            Chef::insert(chef).exec(&db).await?;
        }
        Commands::List => {
            let bakeries = Bakery::find().all(&db).await?;
            let chefs = Chef::find().all(&db).await?;
            println!("Bakeries:\n{:?}\n\nChefs:\n{:?}", bakeries, chefs);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        panic!("{}", err);
    }
}
