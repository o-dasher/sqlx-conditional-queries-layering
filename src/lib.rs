/// Macro to handle query templates for the sqlx-conditional-queries library.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let keehee = [Keehee::OwO, Keehee::UmU, Keehee::UwU]
///     .choose(&mut rand::thread_rng())
///     .cloned()
///     .unwrap_or_default();
///
/// sqlx_conditional_queries_layering::create_conditional_query_as!(
///     $keehee_query,
///     #keehee = match keehee {
///         Keehee::OwO => "owo",
///         Keehee::UmU => "umu",
///         Keehee::UwU => "uwu"
///     }
/// );
/// ```
/// This will generate two macros `_keehee_query` and 'feed__keehee_query'.
/// `_keehee_query` is a template macro that accepts a type, a query and can also be provided with aditional sqlx-conditional-queries arguments.
///
/// You can use it as such:
/// ```
/// keehee_query!(BigID, "DO YOUR QUERY", #hey=match {...})
///     .fetch_one(&pool)
///     .await;
/// ```
///
/// `feed_YOUR_QUERY_NAME` on the other hand is a macro that should be used when you want to add more
/// conditional queries on top of another existing query template.
/// ```
/// feed_query_keehee_query!(
///     $super_duper_query,
///     #oi = match something {
///         Something::Oi => "dumb"
///         Something::Nah => "cool"
///     }
/// );
/// ```
/// This effectively will generate a macros for a keehee_query with the addition of the #oi
/// arguments. This behaviour is recursive, so you can also "feed" this already feeded query with
/// more arguments if you find desireable to do so. You would use `feed_super_duper_query` for
/// that.
///
/// Note: This macro relies on other macros from `sqlx_conditional_queries`.
///
/// # See Also
/// - [`sqlx_conditional_queries`](https://docs.rs/sqlx_conditional_queries)
#[macro_export]
macro_rules! create_conditional_query_as {
    (@$dollar:tt$name:tt, $($conditional_part:tt)*) => {
        #[allow(unused_macros)]
        macro_rules! $name {
            ($type:ty, $query:expr $dollar(, $dollar($more_conditionals:tt)*)?) => {
                sqlx_conditional_queries::conditional_query_as!(
                    $type,
                    $query,
                    $($conditional_part)*
                    $dollar(, $dollar($more_conditionals)*)?
                )
            };
        }

        paste::paste! {
            #[allow(unused_macros)]
            macro_rules! [<feed_query_ $name>] {
                ($feed_dollar:tt$feed_name:tt, $dollar($feed_conditionals:tt)*) => {
                    sqlx_conditional_queries_layering::create_conditional_query_as!(
                        @$feed_dollar$feed_name,
                        $($conditional_part)*,
                        $dollar($feed_conditionals)*
                    )
                };
            }
        }
    };

    ($dollar:tt$name:tt, $($conditional_part:tt)*) => {
        sqlx_conditional_queries_layering::create_conditional_query_as!(@$dollar$name, $($conditional_part)*)
    };
}
