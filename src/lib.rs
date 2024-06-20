#![feature(macro_metavar_expr)]

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

    ($a:ident, $b:ident, $suffix:ident) => {
        paste::paste! {
            sqlx_conditional_queries_layering::merge_sql_query_as!(
                ([<$a $suffix>], [<$b $suffix>]) as [<$a _with_ $b $suffix>]
            )
        }
    };

    ($a:ident, $b:ident) => {
        sqlx_conditional_queries_layering::merge_sql_query_as!($a, $b, _query)
    };
}

pub enum Fall {
    Through,
}
