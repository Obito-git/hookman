use clap::{Parser, Subcommand};
use std::fmt::{Display, Formatter};
use domain::model::persistence::{PersistenceTypeModel, PostgresConfigurationModel, SQLiteConfigurationModel};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct AppConfig {
    #[command(subcommand)]
    pub persistence: Option<PersistenceTypeCommand>,
    #[arg(long, default_value = "localhost")]
    pub host: String,
    #[arg(long, default_value = "4343")]
    pub webhook_port: u16,
    #[arg(long, default_value = "4242")]
    pub ui_api_port: u16,
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Host: {}", self.host)?;
        writeln!(f, "Webhook Port: {}", self.webhook_port)?;
        writeln!(f, "UI/API Port: {}", self.ui_api_port)?;
        write!(f, "Persistence: ")?;
        match &self.persistence {
            Some(PersistenceTypeCommand::InMemory) => {
                writeln!(f, "InMemory")
            }
            Some(PersistenceTypeCommand::SQLiteFile {
                database_name,
                folder_path,
            }) => {
                writeln!(
                    f,
                    "SQLiteFile (database_name: {}, folder_path: {})",
                    database_name, folder_path
                )
            }
            Some(PersistenceTypeCommand::Postgres {
                user,
                password,
                host,
                port,
                database,
            }) => {
                writeln!(
                    f,
                    "Postgres (user: {}, password: {}, host: {}, port: {}, database: {})",
                    user, password, host, port, database
                )
            }
            None => writeln!(f, "InMemory (default)"),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum PersistenceTypeCommand {
    InMemory,
    SQLiteFile {
        #[arg(long)]
        database_name: String,
        #[arg(long, default_value = ".")]
        folder_path: String,
    },
    Postgres {
        #[arg(long)]
        user: String,
        #[arg(long)]
        password: String,
        #[arg(long, default_value = "localhost")]
        host: String,
        #[arg(long, default_value_t = 5432)]
        port: u16,
        #[arg(long)]
        database: String,
    },
}

impl AppConfig {
    pub fn persistence_type(&self) -> PersistenceTypeModel {
        match &self.persistence {
            Some(PersistenceTypeCommand::SQLiteFile {
                database_name,
                folder_path,
            }) => PersistenceTypeModel::SQLiteFile(SQLiteConfigurationModel {
                database_name: database_name.clone(),
                folder_path: folder_path.clone(),
            }),
            Some(PersistenceTypeCommand::Postgres {
                user,
                password,
                host,
                port,
                database,
            }) => PersistenceTypeModel::Postgres(PostgresConfigurationModel {
                user: user.clone(),
                password: password.clone(),
                host: host.clone(),
                port: *port,
                database: database.clone(),
            }),
            _ => PersistenceTypeModel::InMemory,
        }
    }
}
