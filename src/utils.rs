use futures::{stream::FuturesUnordered, Future};
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct AppState { pub pool: PgPool }

#[macro_export]
macro_rules! upsert {
    ($q:ident ; $pool:expr ; $table_name:literal $($column:ident),+ ; conflict($($conflict_column:literal),+)) => {
        {
            let mut column_string = String::new();
            let mut update_string = String::new();
            let mut values_string = String::new();
            let mut idx = 0;
            $( 
                idx += 1;
                column_string.push_str(stringify!($column));
                column_string.push_str(",");
    
                let c = stringify!($column);
                update_string.push_str(&format!("{c} = excluded.{c}"));
                update_string.push_str(",");
    
                values_string.push_str(&format!("${idx}"));
                values_string.push_str(",");
            )+
            let column_string = column_string.strip_suffix(",").unwrap();
            let update_string = update_string.strip_suffix(",").unwrap();
            let values_string = values_string.strip_suffix(",").unwrap();
    
            let mut conflict_string = String::new();
            $( 
                conflict_string.push_str($conflict_column);
                conflict_string.push_str(",");
            )+
            let conflict_string = conflict_string.strip_suffix(",").unwrap();
    
            let table_name_string = $table_name;
    
            let q1 = format!("
            insert into {table_name_string} ({column_string})
            values ({values_string})
            on conflict ({conflict_string}) do update set
            {update_string};
            ");

            for c in q1.chars() {
                $q.push(c);
            }
    
            let q = sqlx::query(&$q);
            $( 
                let q = q.bind($column);
            )+
            q.execute($pool)
        }

    }
}
pub(crate) use upsert; 
