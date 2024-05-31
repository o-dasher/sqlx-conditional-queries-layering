use dotenvy::dotenv;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolOptions, types::BigDecimal, Postgres};

#[derive(Clone, Copy, Default)]
enum Keehee {
    #[default]
    OwO,
    UmU,
    UwU,
}

struct BigID {
    id: BigDecimal,
}

#[derive(Serialize, Deserialize)]
struct DotenvConfig {
    pub database_url: String,
}

fn keehoo() -> Keehee {
    [Keehee::OwO, Keehee::UmU, Keehee::UwU]
        .choose(&mut rand::thread_rng())
        .cloned()
        .unwrap_or_default()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    dotenv().ok();

    let config = envy::from_env::<DotenvConfig>()?;
    let pool = PoolOptions::<Postgres>::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    let keehaa = keehoo();
    let keehee = keehoo();

    sqlx_conditional_queries_layering::create_conditional_query_as!(
        $keehee_query,
        #keehee = match keehaa {
            Keehee::OwO => "owo",
            Keehee::UmU => "umu",
            Keehee::UwU => "uwu"
        }
    );

    feed_query_keehee_query!(
        $lewdy,
        #lewdiness = match keehee {
            Keehee::OwO => "owo",
            Keehee::UmU => "umu",
            Keehee::UwU => "uwu"
        }
    );

    let result = lewdy!(BigID, "INSERT INTO {#keehee} DEFAULT VALUES RETURNING id",)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(v) => {
            println!("Inserted {}", v.id);
        }
        Err(e) => {
            println!("Inserted nothing and got this: {e}")
        }
    };

    Ok(())
}
