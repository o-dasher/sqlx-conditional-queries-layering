# `sqlx_conditional_queries_layering`

This library provides a macro for handling query templates in conjunction with the `sqlx_conditional_queries` library. It simplifies the creation of SQL queries with conditional parameters.

### Basic Example

```rust
let keehee = [Keehee::OwO, Keehee::UmU, Keehee::UwU]
    .choose(&mut rand::thread_rng())
    .cloned()
    .unwrap_or_default();

create_conditional_query_as!(
    $,
    keehee_query,
    #keehee = match keehee {
        Keehee::OwO => "owo",
        Keehee::UmU => "umu",
        Keehee::UwU => "uwu"
    }
);
```

This will generate two macros: `_keehee_query` and `feed_keehee_query`.

### Using Generated Macros

`_keehee_query` is a template. You can use it to do an sqlx query:

```rust
keehee_query!(BigID, "DO YOUR QUERY", #hey=match {...})
    .fetch_one(&pool)
    .await;
```

`feed_YOUR_QUERY_NAME` is used to add more conditional queries to an existing template:

```rust
feed_query_keehee_query!(
    $,
    super_duper_query,
    #oi = match something {
        Something::Oi => "dumb",
        Something::Nah => "cool"
    }
);
```

This will generate a macro for `keehee_query` with the additional `#oi` argument. You can continue adding more arguments recursively.

## Note
This macro relies on other macros from [`sqlx_conditional_queries`](https://docs.rs/sqlx_conditional_queries).

## See Also

- [`sqlx_conditional_queries`](https://docs.rs/sqlx_conditional_queries)

