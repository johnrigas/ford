use proc_macro::TokenStream;
use quote::quote;
use syn;





#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}


// let result_future = upsert! ( 
//     "espn.games", 
//     game_id, over_under, over_odds, under_odds, opening_over_under, opening_over_odds, opening_under_odds, start_time, 
//     conflict("game_id", "over_under") 
// );
    
// let q = "
// insert into espn.games (game_id, over_under, over_odds, under_odds, opening_over_under, opening_over_odds, opening_under_odds, start_time)
// values ($1, $2, $3, $4, $5, $6, $7, $8)
// on conflict (game_id) do update set 
// over_under = excluded.over_under,
// over_odds = excluded.over_odds,
// under_odds = excluded.under_odds,
// opening_over_under = excluded.opening_over_under,
// opening_over_odds = excluded.opening_over_odds,
// opening_under_odds = excluded.opening_under_odds,
// start_time = excluded.start_time;
// ";

// let _result = sqlx::query(q)
// .bind(&b["header"]["id"].as_str())
// .bind(&pickcenter.unwrap()["current"]["total"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<f32>().unwrap())))
// .bind(&pickcenter.unwrap()["current"]["over"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
// .bind(&pickcenter.unwrap()["current"]["under"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
// .bind(&pickcenter.unwrap()["open"]["total"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<f32>().unwrap())))
// .bind(&pickcenter.unwrap()["open"]["over"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
// .bind(&pickcenter.unwrap()["open"]["under"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
// .bind(&day.day)
// .execute(&state.pool).await.unwrap();