# `sqlx_conditional_queries_layering`

This library provides a macro for handling query templates in conjunction with the `sqlx_conditional_queries` library. It simplifies the creation of SQL queries with conditional parameters.

### Basic Example

```rust
let keehee = [Keehee::OwO, Keehee::UmU, Keehee::UwU]
    .choose(&mut rand::thread_rng())
    .cloned()
    .unwrap_or_default();

create_conditional_query_as!(
    $keehee_query,
    #keehee = match keehee {
        Keehee::OwO => "owo",
        Keehee::UmU => "umu",
        Keehee::UwU => "uwu"
    }
);
```

This will generate an `keehee_query` macro.

### Using the generated query

`keehee_query` is a template. You can use it to do an sqlx query:

```rust
keehee_query!(BigID, "DO YOUR QUERY", #hey=match {...})
    .fetch_one(&pool)
    .await;
```

We can further extend the template with aditional variables using the macro `supply_sql_variables_to_query_as`.

```rust
supply_sql_variables_to_query_as!(
    keehee_query as some_query,
    #name = match Fall::Through {
        _ => "{keehee_name}",
    }
);
```
In this example we create `some_query`, which is another macro that have the same template variables as `keehee_query` with the addition of `#name`.

### It does not stop here though!
We can merge two queries into one through:
```rs
merge_sql_query_as!((a, b) as argsception);
```
This will merge all the template variables of `a` and `b` into a single `argsception` query!

## Note
This macro relies on other macros from [`sqlx_conditional_queries`](https://docs.rs/sqlx_conditional_queries).

### Dependencies
You need to enable the `macro_metavar_expr` feature to use this library:
```rs
#![feature(macro_metavar_expr)]
```
You will also need to add the following dependencies to `Cargo.toml`: `sqlx_conditional_queries`, `paste`.
You can do so easily, through:
```
cargo add sqlx_conditional_queries
cargo add paste
```

### See Also

- [`sqlx_conditional_queries`](https://docs.rs/sqlx_conditional_queries)

