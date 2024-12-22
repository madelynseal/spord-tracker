use crate::CONFIG;
use async_sqlite::{Client, ClientBuilder, JournalMode, Pool, PoolBuilder};
use chrono::DateTime;
use models::{SpordRecord, SpordState};

pub mod models;

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
            username TEXT PRIMARY KEY,
            password TEXT NOT NULL,
            enabled BOOL NOT NULL,
            lastlogin INTEGER
            )",
                (),
            )
        })
        .await?;

    client
        .conn(|conn| {
            conn.execute(
                "CREATE TABLE spords (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    phone TEXT,
                    email TEXT,
                    part TEXT,
                    state INTEGER NOT NULL,
                    created INTEGER NOT NULL,
                    received INTEGER,
                    comments TEXT
                )",
                (),
            )
        })
        .await?;

    Ok(())
}

pub async fn user_login(
    client_opt: Option<Client>,
    username: &str,
    password: &str,
) -> Result<bool> {
    let client = open_if_needed(client_opt).await?;

    let sqlusername = username.to_owned();
    let sqlpswd: String = client
        .conn(move |conn| {
            conn.query_row(
                "SELECT password FROM auth WHERE username=?",
                [&sqlusername],
                |row| row.get(0),
            )
        })
        .await?;

    if bcrypt::verify(password, &sqlpswd)? {
        //TODO: update last logged in state

        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn user_create(client_opt: Option<Client>, username: &str, password: &str) -> Result<()> {
    let client = open_if_needed(client_opt).await?;

    let sqlpswd = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    let sqlusername = username.to_owned();

    client
        .conn(move |conn| {
            conn.execute(
                "INSERT INTO auth (username, password, enabled) VALUES(?1, ?2, ?3)",
                (&sqlusername, &sqlpswd, &true),
            )
        })
        .await?;

    Ok(())
}

// Used by the cli to create users
pub async fn user_create_console(
    client_opt: Option<Client>,
    username: Option<String>,
) -> Result<()> {
    let username = if let Some(username) = username {
        username
    } else {
        crate::prompt_user_input("Username: ")?
    };

    let password1 = rpassword::prompt_password("Password: ")?;
    let password2 = rpassword::prompt_password("Password again: ")?;

    if password1 == password2 {
        user_create(client_opt, &username, &password1).await?;
        println!("User {} created!", username);
    } else {
        panic!("Passwords do not match! Please try again");
    }

    Ok(())
}

pub async fn spord_create(client_opt: Option<Client>, spord: SpordRecord) -> Result<()> {
    let client = open_if_needed(client_opt).await?;

    client
        .conn(move |conn| {
            conn.execute(
                "INSERT INTO spords 
        (name, phone, email, part, state, created, received, comments)
        VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                (
                    &spord.customer_name,
                    &spord.customer_phone,
                    &spord.customer_email,
                    &spord.part,
                    spord.state.as_sql(),
                    spord.creation_date.timestamp(),
                    spord.received_date_unix(),
                    &spord.comments,
                ),
            )
        })
        .await?;

    Ok(())
}

pub async fn spord_update(client_opt: Option<Client>, spord: SpordRecord) -> Result<()> {
    let client = open_if_needed(client_opt).await?;

    client.conn(move |conn| {

        conn.execute("UPDATE spords SET name=?1, phone=?2, email=?3, part=?4, state=?5, created=?6, received=?7, comments=?8 WHERE id=?9;", 
        (&spord.customer_name, &spord.customer_phone, &spord.customer_email
            , &spord.part, spord.state.as_sql(), spord.creation_date.timestamp(), spord.received_date_unix(), &spord.comments, spord.id))
    }).await?;
    Ok(())
}

pub async fn spord_get_all(client_opt: Option<Client>) -> Result<Vec<SpordRecord>> {
    let client = open_if_needed(client_opt).await?;

    let myspords = client
        .conn(|conn| {
            let mut stmt = conn.prepare("SELECT * FROM spords")?;

            let spord_iter = stmt.query_map([], |row| {
                Ok(SpordRecord {
                    id: row.get(0)?,
                    customer_name: row.get(1)?,
                    customer_phone: row.get(2)?,
                    customer_email: row.get(3)?,
                    part: row.get(4)?,
                    state: SpordState::from_sql(row.get(5)?),
                    creation_date: DateTime::from_timestamp(row.get(6)?, 0).unwrap(),
                    received_date: None,
                    comments: row.get(8)?,
                })
            })?;
            let mut spords = vec![];
            for spord in spord_iter {
                if spord.is_ok() {
                    spords.push(spord.unwrap());
                } else {
                    warn!("Error fetch spord record: {:?}", spord.unwrap_err());
                }
            }
            Ok(spords)
        })
        .await?;

    Ok(myspords)
}
