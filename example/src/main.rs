#![feature(macro_metavar_expr)]

use dotenvy::dotenv;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolOptions, types::BigDecimal, Postgres};
use sqlx_conditional_queries_layering::Fall;
use sqlx_conditional_queries_layering::{merge_sql_query_as, supply_sql_variables_to_query_as};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    dotenv().ok();

    let config = envy::from_env::<DotenvConfig>()?;
    let pool = PoolOptions::<Postgres>::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    let keehee = [Keehee::OwO, Keehee::UmU, Keehee::UwU]
        .choose(&mut rand::thread_rng())
        .cloned()
        .unwrap_or_default();

    sqlx_conditional_queries_layering::create_conditional_query_as!(
        keehee_query,
        #keehee = match keehee {
            Keehee::OwO => "owo",
            Keehee::UmU => "umu",
            Keehee::UwU => "uwu"
        }
    );

    let keehee_name = "cool";

    // Here we are suppling the #name variable to the keehee_query
    // and we aliase the resulting query to another query name
    supply_sql_variables_to_query_as!(
        keehee_query as lewdy_query,
        #name = match Fall::Through {
            _ => "{keehee_name}",
        }
    );

    // Do not do this lol, it is for the sake of an example!
    sqlx_conditional_queries_layering::create_conditional_query_as!(
        return_id_query,
        #return_id = match Fall::Through {
            _ => "RETURNING id"
        }
    );

    // We can merge existing query templates!
    // with an explicit given name for the new merged query
    // merge_sql_query_as!((lewdy_query, return_id_query) as argsception);
    // or we can do just as below if both queries follow the "query" suffix.
    merge_sql_query_as!(lewdy, return_id);

    let result = lewdy_with_return_id_query!(
        BigID,
        "INSERT INTO {#keehee} (name) VALUES ({#name}) {#return_id}",
    )
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
