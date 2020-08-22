use std::convert::TryInto;

use chrono::{DateTime, Utc};
use sqlx::PgPool;

use thiserror::Error;

type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("{0}")]
    SqlxDatabaseError(#[from] sqlx::Error),

    #[error("FromDbTypeError: failed to convert {0}")]
    FromDbTypeError(String),
}

#[derive(Debug)]
pub struct Appointment {
    pub description: String,
    pub stupid_id_to_convert: u32,
    pub timestamp: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> DatabaseResult<()> {
    std::env::set_var("RUST_LOG", "error");
    env_logger::init();

    // export DATABASE_URL="postgres://olga:dummy@localhost/habit_db"
    // despite a pool and a connection sqlx ALSO needs
    // DATABASE_URL to be set to run its type checking logic
    let pool = PgPool::connect("postgres://olga:dummy@localhost/habit_db").await?;

    add_appointment(&pool, "test".to_owned()).await?;

    let current_appointments = fetch_appointments_manual(&pool).await?;

    for appointment in current_appointments {
        println!("{:?}", appointment);
    }

    Ok(())
}

async fn add_appointment(pool: &PgPool, description: String) -> DatabaseResult<i64> {
    let rec = sqlx::query!(
        r#"
INSERT INTO appointments (description, stupid_id_to_convert)
VALUES ($1, $2)
RETURNING id
        "#,
        description,
        3 // Change this to a negative value to see the type conversion error, i.e.:
        /*
        [2020-08-22T15:37:24Z ERROR planner_tests] out of range integral type conversion attempted
        Error: FromDbTypeError("stupid_id_to_convert")
        */
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

async fn fetch_appointments_manual(pool: &PgPool) -> DatabaseResult<Vec<Appointment>> {
    let rows =
        sqlx::query!("SELECT description, stupid_id_to_convert, timestamp FROM appointments")
            .fetch_all(pool)
            .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            Ok(Appointment {
                description: row.description,
                stupid_id_to_convert: row.stupid_id_to_convert.try_into().map_err(|err| {
                    log::error!("{}", err);

                    DatabaseError::FromDbTypeError("stupid_id_to_convert".to_owned())
                })?,
                timestamp: row.timestamp,
            })
        })
        .collect::<Result<Vec<Appointment>, DatabaseError>>()?)
}

// Separate fetches from mappers?

/*
// If Appointment was a simpler type without the stupid_id_to_convert
async fn fetch_appointments(pool: &PgPool) -> sqlx::Result<Vec<Appointment>> {
    Ok(sqlx::query_as!(
        Appointment,
        "SELECT description, timestamp FROM appointments")
    .fetch_all(pool)
    .await?)
}
*/
