#![feature(macro_metavar_expr)]

/// Macro to handle query templates for the sqlx-conditional-queries library.
///
/// # Examples
///
/// We need to enable macro_metavar_expr unstable feature
/// Basic usage:
/// ```
/// #![feature(macro_metavar_expr)]
/// let keehee = [Keehee::OwO, Keehee::UmU, Keehee::UwU]
///     .choose(&mut rand::thread_rng())
///     .cloned()
///     .unwrap_or_default();
///
/// sqlx_conditional_queries_layering::create_conditional_query_as!(
///     keehee_query,
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
///     super_duper_query,
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
///
/// Although it would be nice to be able to use the query on multiple files,
/// this unfortunately does not seem to be possible the way rust is structured.
/// https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/builtin/static.MACRO_EXPANDED_MACRO_EXPORTS_ACCESSED_BY_ABSOLUTE_PATHS.html
#[macro_export]
macro_rules! create_conditional_query_as {
    ($name:tt, $($conditional_part:tt)*) => {
        #[allow(unused_macros)]
        macro_rules! $name {
            ($type:ty, $query:expr $$(, $$($more_conditionals:tt)*)?) => {
                sqlx_conditional_queries::conditional_query_as!(
                    $type,
                    $query,
                    $($conditional_part)*
                    $$(, $$($more_conditionals)*)?
                )
            };
        }

        paste::paste! {
            #[allow(unused_macros)]
            macro_rules! [<_DO_NOT_USE_EXPLICITLY_ $name>] {
                ($feed_name:tt, $$($feed_conditionals:tt)*) => {
                    sqlx_conditional_queries_layering::create_conditional_query_as!(
                        $feed_name,
                        $($conditional_part)*,
                        $$($feed_conditionals)*
                    );
                };
            }

            #[allow(unused_macros)]
            macro_rules! [<_$name _DO_NOT_USE_EXPLICITLY>] {
                ($existing_query:ident, $feed_name:tt) => {
                    $existing_query!(
                        $feed_name,
                        $($conditional_part)*
                    );
                }
            }
        }
    };
}

#[macro_export]
macro_rules! supply_sql_variables_to_query_as {
    ($query_macro:ident as $as:ident, $($conditional_part:tt)*) => {
        paste::paste! {
            [<_DO_NOT_USE_EXPLICITLY_ $query_macro>]!($as, $($conditional_part)*)
        }
    };
}

#[macro_export]
macro_rules! merge_sql_query_as {
    (($a:ident, $b:ident) as $as:ident) => {
        paste::paste! {
            [<_ $a _DO_NOT_USE_EXPLICITLY>]!(
                [<_DO_NOT_USE_EXPLICITLY_ $b>],
                $as
            )
        }
    };
}

pub enum Fall {
    Through,
}
