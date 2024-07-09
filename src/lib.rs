#[macro_export]
macro_rules! create_conditional_query_as {
    ($dollar:tt$name:tt, $($conditional_part:tt)*) => {
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
            macro_rules! [<_DO_NOT_USE_EXPLICITLY_ $name>] {
                ($inner_dollar:tt$feed_name:tt, $dollar($feed_conditionals:tt)*) => {
                    sqlx_conditional_queries_layering::create_conditional_query_as!(
                        $inner_dollar$feed_name,
                        $($conditional_part)*,
                        $dollar($feed_conditionals)*
                    );
                };
            }

            #[allow(unused_macros)]
            macro_rules! [<_$name _DO_NOT_USE_EXPLICITLY>] {
                ($inner_dollar:tt$existing_query:ident, $feed_name:tt) => {
                    $existing_query!(
                        $inner_dollar$feed_name,
                        $($conditional_part)*
                    );
                }
            }
        }
    };
}

#[macro_export]
macro_rules! supply_sql_variables_to_query_as {
    ($dollar:tt$query_macro:ident as $as:ident, $($conditional_part:tt)*) => {
        paste::paste! {
            [<_DO_NOT_USE_EXPLICITLY_ $query_macro>]!($dollar$as, $($conditional_part)*)
        }
    };
}

#[macro_export]
macro_rules! merge_sql_query_as {
    ($dollar:tt($a:ident, $b:ident) as $as:ident) => {
        paste::paste! {
            [<_ $a _DO_NOT_USE_EXPLICITLY>]!(
                $dollar[<_DO_NOT_USE_EXPLICITLY_ $b>],
                $as
            )
        }
    };

    ($dollar:tt@$suffix:ident, $a:ident, $b:ident) => {
        paste::paste! {
            sqlx_conditional_queries_layering::merge_sql_query_as!(
                $dollar([<$a $suffix>], [<$b $suffix>]) as [<$a _with_ $b $suffix>]
            )
        }
    };

    ($dollar:tt@$suffix:ident, $a:ident, $b:ident $(, $c:ident)*) => {
        sqlx_conditional_queries_layering::merge_sql_query_as!($dollar@$suffix, $a, $b);
        paste::paste! {
            sqlx_conditional_queries_layering::merge_sql_query_as!(
                @$suffix,
                [<$a _with_ $b>]
                $(, $c)*
            );
        }
    };

    ($dollar:tt($a:ident, $b:ident)) => {
        sqlx_conditional_queries_layering::merge_sql_query_as!($dollar@_query, $a, $b)
    };

    ($dollar:tt($a:ident, $b:ident $(, $c:ident)*)) => {
        sqlx_conditional_queries_layering::merge_sql_query_as!($dollar@_query, $a, $b $(, $c)*)
    }
}

pub enum Fall {
    Through,
}
