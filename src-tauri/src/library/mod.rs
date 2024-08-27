pub mod entry;
pub mod error;
pub mod tag;

use std::path::{Path, PathBuf};

pub use error::Error;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Location {
    path: PathBuf,
    name: Option<String>,
}

impl Location {
    pub fn new<P: Into<PathBuf>, S: Into<String>>(path: P, name: Option<S>) -> Location {
        Location {
            path: path.into(),
            name: name.map(|s| s.into()),
        }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Clone, Debug)]
pub struct Library {
    home: PathBuf,
    pub locations: Vec<Location>,
    pool: SqlitePool,
}

impl Library {
    pub async fn new<P: Into<PathBuf>>(home: P, locations: Vec<Location>) -> Result<Library> {
        let home = home.into();
        let pool = Self::connect(&home, true).await?;

        Ok(Library {
            home,
            locations,
            pool,
        })
    }

    pub async fn open<P: Clone + Into<PathBuf>>(home: P) -> Result<Library> {
        let pool = Self::connect(home.clone(), false).await?;
        let locations = sqlx::query_as!(Location, "SELECT path, name FROM location")
            .fetch_all(&pool)
            .await?;

        Ok(Library {
            home: home.into(),
            locations,
            pool,
        })
    }

    async fn connect<P: Into<PathBuf>>(home: P, create: bool) -> Result<SqlitePool> {
        let filename = home.into().join("library.sqlite");

        // sqlx does not provide a mechanism to error if already created
        //
        // there is a potential for a race condition, such a scenario would
        // mean creating a library in the same location right after this check
        if !create && filename.exists() {
            return Err(Error::LibraryExists(filename));
        }

        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .filename(filename)
                .create_if_missing(create),
        )
        .await?;
        // sqlx::migrate!().run(&pool).await?;

        Ok(pool)
    }
}
