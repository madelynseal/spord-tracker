use crate::CONFIG;
use async_sqlite::{Client, ClientBuilder, JournalMode, Pool, PoolBuilder};

#[derive(Debug, Error)]
pub enum SqlError {
    #[error("Sql(IO({0:?}))")]
    Io(#[from] std::io::Error),

    #[error("Sql(Sqlite({0:?}))")]
    Sqlite(#[from] async_sqlite::Error),

    #[error("Sql(Bcrypt({0:?}))")]
    Bcrypt(#[from] bcrypt::BcryptError),
}
type Result<T> = std::result::Result<T, SqlError>;

pub async fn open_connection() -> Result<Client> {
    let client = ClientBuilder::new()
        .path(&CONFIG.sql.location)
        .journal_mode(JournalMode::Wal)
        .open()
        .await?;

    Ok(client)
}

async fn open_if_needed(client_opt: Option<Client>) -> Result<Client> {
    let client = if let Some(client) = client_opt {
        client
    } else {
        open_connection().await?
    };

    Ok(client)
}

pub async fn check_initialized() -> Result<()> {
    if !std::fs::exists(&CONFIG.sql.location)? {
        initialize(None).await?;
    }

    Ok(())
}

async fn initialize(client_opt: Option<Client>) -> Result<()> {
    let client = open_if_needed(client_opt).await?;

    client
        .conn(|conn| {
            conn.execute(
                "CREATE TABLE auth (
            name TEXT PRIMARY KEY,
            pswd TEXT NOT NULL,
            enabled BOOL NOT NULL,
            lastlogin INT
        )
       ",
                (),
            )
        })
        .await?;

    Ok(())
}

pub async fn user_login(client_opt: Option<Client>) -> Result<bool> {
    let client = open_if_needed(client_opt).await?;

    unimplemented!();
    Ok(false)
}

pub async fn user_create(
    client_opt: Option<Client>,
    username: String,
    password: &str,
) -> Result<()> {
    let client = open_if_needed(client_opt).await?;

    let sqlpswd = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    client
        .conn(move |conn| {
            conn.execute(
                "INSERT INTO auth (name, pswd, enabled) VALUES(?1, ?2, ?3)",
                (&username, &sqlpswd, &true),
            )
        })
        .await?;

    Ok(())
}
