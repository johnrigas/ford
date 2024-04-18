use axum::extract::State;
use axum_macros::debug_handler;
use reqwest::Method;
use scraper::{Html, Selector};
use serde::Deserialize;
use crate::utils::AppState;
use crate::refresh::utils;
use std::collections::HashMap;
use reqwest::cookie::Jar;
use std::fs;
use futures::StreamExt;
use chrono;



#[debug_handler]
pub async fn refresh_vegas(State(state): State<AppState>) -> &'static str {

    let mut v = Vec::new(); 

    {
        let url = "https://www.vegasinsider.com/college-basketball/odds/las-vegas/";
        let mut headers = reqwest::header::HeaderMap::new();
        let resp = utils::api(url, Method::GET, headers, vec![], {}, Option::<Jar>::None).await.text().await.unwrap();
        let doc = Html::parse_document(&resp);

            
        let spread_tb_selector = Selector::parse("#odds-table-spread--0").unwrap();
        let moneyline_tb_selector = Selector::parse("#odds-table-moneyline--0").unwrap();
        let totals_tb_selector = Selector::parse("#odds-table-total--0").unwrap();
        let spread_tb = doc.select(&spread_tb_selector).next().unwrap();
        let moneyline_tb = doc.select(&moneyline_tb_selector).next().unwrap();
        let totals_tb = doc.select(&totals_tb_selector).next().unwrap();

        let tr_selector = Selector::parse("tr").unwrap();
        let mut spread_trs = spread_tb.select(&tr_selector);
        let mut moneyline_trs = moneyline_tb.select(&tr_selector);
        let mut totals_trs = totals_tb.select(&tr_selector);
        let game_team_selector = Selector::parse(".game-team").unwrap();
        let game_odds_selector = Selector::parse(".game-odds").unwrap();
        let data_value_selector = Selector::parse(".data-value").unwrap();
        let data_moneyline_selector = Selector::parse(".data-moneyline").unwrap();
        let data_odds_selector = Selector::parse(".data-odds").unwrap();
        let game_time_selector = Selector::parse(".game-time").unwrap();
        let book_logo_selector = Selector::parse(".book-logo").unwrap();

        let a_selector = Selector::parse("a").unwrap();
        let img_selector = Selector::parse("img").unwrap();
        let span_selector = Selector::parse("span").unwrap();

        // spreads
        loop {
            let line_type = "spread";

            let game_time_books_tr_option = spread_trs.next();
            let team_1_tr_option = spread_trs.next();
            let team_2_tr_option = spread_trs.next();
            let matchup_tr_option = spread_trs.next();

            if !game_time_books_tr_option.is_some() {
                break;
            };

            let game_time_books_tr = game_time_books_tr_option.unwrap();
            let team_1_tr = team_1_tr_option.unwrap();
            let team_2_tr = team_2_tr_option.unwrap();
            let matchup_tr = matchup_tr_option.unwrap();

            if !matchup_tr.html().contains("Matchup") {
                continue;
            };

            let default_time_string = chrono::offset::Local::now().to_string();
            let game_time = game_time_books_tr.select(&game_time_selector).next().unwrap().select(&span_selector).next().unwrap().attr("data-value").unwrap_or(&default_time_string);

            let book_logos = game_time_books_tr.select(&book_logo_selector);
            let mut team_1_odds_divs = team_1_tr.select(&game_odds_selector);
            let mut team_2_odds_divs = team_2_tr.select(&game_odds_selector);

            for book_logo in book_logos {

                let team_1_span = team_1_tr.select(&game_team_selector).next().unwrap().select(&span_selector).next().unwrap();
                let team_1_a = team_1_span.select(&a_selector).next().unwrap();
                let team_1_name = team_1_a.attr("aria-label").unwrap();
                let team_1_abbreviation = team_1_a.attr("data-abbr").unwrap();
                let team_1_id_span = team_1_span.select(&span_selector).next().unwrap().clone();
                let team_1_id = team_1_id_span.inner_html().trim().to_string();
        
                let team_2_span = team_2_tr.select(&game_team_selector).next().unwrap().select(&span_selector).next().unwrap();
                let team_2_a = team_2_span.select(&a_selector).next().unwrap();
                let team_2_name = team_2_a.attr("aria-label").unwrap();
                let team_2_abbreviation = team_2_a.attr("data-abbr").unwrap();
                let team_2_id_span = team_2_span.select(&span_selector).next().unwrap().clone();
                let team_2_id = team_2_id_span.inner_html().trim().to_string();

                let sportsbook = book_logo.select(&img_selector).next().unwrap().attr("alt").unwrap();

                let team_1_odds_tr = team_1_odds_divs.next().unwrap();
                let team_2_odds_tr = team_2_odds_divs.next().unwrap();

                let team_1_handicap_html = team_1_odds_tr.select(&data_value_selector).next().unwrap().inner_html();
                let team_1_handicap = team_1_handicap_html.trim();

                let team_2_handicap_html = team_2_odds_tr.select(&data_value_selector).next().unwrap().inner_html();
                let team_2_handicap = team_2_handicap_html.trim();

                if team_1_handicap == "N/A" || team_2_handicap == "N/A" {
                    continue;
                }

                let team_1_odds_html = team_1_odds_tr.select(&data_odds_selector).next().unwrap().inner_html();
                let team_1_odds = team_1_odds_html.trim();

                let team_2_odds_html = team_2_odds_tr.select(&data_odds_selector).next().unwrap().inner_html();
                let team_2_odds = team_2_odds_html.trim();


                let q = "
                insert into vegas.lines (game_time, team_1_id, team_1_name, team_1_abbreviation, team_2_id, team_2_name, team_2_abbreviation, sportsbook, line_type, 
                    team_1_handicap, team_1_odds, team_2_handicap, team_2_odds)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                on conflict (team_1_id, team_2_id, sportsbook, game_time, line_type) do update set 
                team_1_name = excluded.team_1_name,
                team_1_abbreviation = excluded.team_1_abbreviation,
                team_2_name = excluded.team_2_name,
                team_2_abbreviation = excluded.team_2_abbreviation,
                team_1_handicap = excluded.team_1_handicap,
                team_1_odds = excluded.team_1_odds,
                team_2_handicap = excluded.team_2_handicap,
                team_2_odds = excluded.team_2_odds;
                ";

                let result = sqlx::query(q)
                .bind(game_time.to_string())
                .bind(team_1_id)
                .bind(team_1_name.to_string())
                .bind(team_1_abbreviation.to_string())
                .bind(team_2_id)
                .bind(team_2_name.to_string())
                .bind(team_2_abbreviation.to_string())
                .bind(sportsbook.to_string())
                .bind(line_type)
                .bind(team_1_handicap.parse::<f32>().unwrap_or(0.0))
                .bind(team_1_odds.parse::<i32>().unwrap_or(-100))
                .bind(team_2_handicap.parse::<f32>().unwrap_or(0.0))
                .bind(team_2_odds.parse::<i32>().unwrap_or(-100))
                .execute(&state.pool);

                v.push(result);

            }

        };

        // totals
        loop {
            let line_type = "total";

            let game_time_books_tr_option = totals_trs.next();
            let team_1_tr_option = totals_trs.next();
            let team_2_tr_option = totals_trs.next();
            let matchup_tr_option = totals_trs.next();

            if !game_time_books_tr_option.is_some() {
                break;
            };

            let game_time_books_tr = game_time_books_tr_option.unwrap();
            let team_1_tr = team_1_tr_option.unwrap();
            let team_2_tr = team_2_tr_option.unwrap();
            let matchup_tr = matchup_tr_option.unwrap();

            if !matchup_tr.html().contains("Matchup") {
                continue;
            };

            let default_time_string = chrono::offset::Local::now().to_string();
            let game_time = game_time_books_tr.select(&game_time_selector).next().unwrap().select(&span_selector).next().unwrap().attr("data-value").unwrap_or(&default_time_string);

            let book_logos = game_time_books_tr.select(&book_logo_selector);
            let mut team_1_odds_divs = team_1_tr.select(&game_odds_selector);
            let mut team_2_odds_divs = team_2_tr.select(&game_odds_selector);

            for book_logo in book_logos {


                let team_1_span = team_1_tr.select(&game_team_selector).next().unwrap().select(&span_selector).next().unwrap();
                let team_1_name = "Over";
                let team_1_abbreviation = "Over";
                let team_1_id_span = team_1_span.select(&span_selector).next().unwrap().clone();
                let team_1_id = team_1_id_span.inner_html().trim().to_string();
        
                let team_2_span = team_2_tr.select(&game_team_selector).next().unwrap().select(&span_selector).next().unwrap();
                let team_2_name = "Under";
                let team_2_abbreviation = "Under";
                let team_2_id_span = team_2_span.select(&span_selector).next().unwrap().clone();
                let team_2_id = team_2_id_span.inner_html().trim().to_string();

                let sportsbook = book_logo.select(&img_selector).next().unwrap().attr("alt").unwrap();

                let team_1_odds_tr = team_1_odds_divs.next().unwrap();
                let team_2_odds_tr = team_2_odds_divs.next().unwrap();

                let team_1_handicap_html = team_1_odds_tr.select(&data_value_selector).next().unwrap().inner_html();
                let team_1_handicap = team_1_handicap_html.trim();

                let team_2_handicap_html = team_2_odds_tr.select(&data_value_selector).next().unwrap().inner_html();
                let team_2_handicap = team_2_handicap_html.trim();

                if team_1_handicap == "N/A" || team_2_handicap == "N/A" {
                    continue;
                }

                let team_1_odds_html = team_1_odds_tr.select(&data_odds_selector).next().unwrap().inner_html();
                let team_1_odds = team_1_odds_html.trim();

                let team_2_odds_html = team_2_odds_tr.select(&data_odds_selector).next().unwrap().inner_html();
                let team_2_odds = team_2_odds_html.trim();


                let q = "
                insert into vegas.lines (game_time, team_1_id, team_1_name, team_1_abbreviation, team_2_id, team_2_name, team_2_abbreviation, sportsbook, line_type, 
                    team_1_handicap, team_1_odds, team_2_handicap, team_2_odds)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                on conflict (team_1_id, team_2_id, sportsbook, game_time, line_type) do update set 
                team_1_name = excluded.team_1_name,
                team_1_abbreviation = excluded.team_1_abbreviation,
                team_2_name = excluded.team_2_name,
                team_2_abbreviation = excluded.team_2_abbreviation,
                team_1_handicap = excluded.team_1_handicap,
                team_1_odds = excluded.team_1_odds,
                team_2_handicap = excluded.team_2_handicap,
                team_2_odds = excluded.team_2_odds;
                ";

                let result = sqlx::query(q)
                .bind(game_time.to_string())
                .bind(team_1_id)
                .bind(team_1_name.to_string())
                .bind(team_1_abbreviation.to_string())
                .bind(team_2_id)
                .bind(team_2_name.to_string())
                .bind(team_2_abbreviation.to_string())
                .bind(sportsbook.to_string())
                .bind(line_type)
                .bind((&team_1_handicap[1..]).parse::<f32>().unwrap_or(0.0))
                .bind(team_1_odds.parse::<i32>().unwrap_or(-100))
                .bind((&team_2_handicap[1..]).parse::<f32>().unwrap_or(0.0))
                .bind(team_2_odds.parse::<i32>().unwrap_or(-100))
                .execute(&state.pool);

                v.push(result);

            }

        };


        // moneylines
        loop {
            let line_type = "moneyline";

            let game_time_books_tr_option = moneyline_trs.next();
            let team_1_tr_option = moneyline_trs.next();
            let team_2_tr_option = moneyline_trs.next();
            let matchup_tr_option = moneyline_trs.next();

            if !game_time_books_tr_option.is_some() {
                break;
            };

            let game_time_books_tr = game_time_books_tr_option.unwrap();
            let team_1_tr = team_1_tr_option.unwrap();
            let team_2_tr = team_2_tr_option.unwrap();
            let matchup_tr = matchup_tr_option.unwrap();

            if !matchup_tr.html().contains("Matchup") {
                continue;
            };

            let default_time_string = chrono::offset::Local::now().to_string();
            let game_time = game_time_books_tr.select(&game_time_selector).next().unwrap().select(&span_selector).next().unwrap().attr("data-value").unwrap_or(&default_time_string);

            let book_logos = game_time_books_tr.select(&book_logo_selector);
            let mut team_1_odds_divs = team_1_tr.select(&game_odds_selector);
            let mut team_2_odds_divs = team_2_tr.select(&game_odds_selector);

            for book_logo in book_logos {

                let team_1_span = team_1_tr.select(&game_team_selector).next().unwrap().select(&span_selector).next().unwrap();
                let team_1_a = team_1_span.select(&a_selector).next().unwrap();
                let team_1_name = team_1_a.attr("aria-label").unwrap();
                let team_1_abbreviation = team_1_a.attr("data-abbr").unwrap();
                let team_1_id_span = team_1_span.select(&span_selector).next().unwrap().clone();
                let team_1_id = team_1_id_span.inner_html().trim().to_string();
        
                let team_2_span = team_2_tr.select(&game_team_selector).next().unwrap().select(&span_selector).next().unwrap();
                let team_2_a = team_2_span.select(&a_selector).next().unwrap();
                let team_2_name = team_2_a.attr("aria-label").unwrap();
                let team_2_abbreviation = team_2_a.attr("data-abbr").unwrap();
                let team_2_id_span = team_2_span.select(&span_selector).next().unwrap().clone();
                let team_2_id = team_2_id_span.inner_html().trim().to_string();

                let sportsbook = book_logo.select(&img_selector).next().unwrap().attr("alt").unwrap();


                let team_1_odds_tr = team_1_odds_divs.next().unwrap();
                let team_2_odds_tr = team_2_odds_divs.next().unwrap();

                let team_1_odds_html = team_1_odds_tr.select(&data_moneyline_selector).next();
                let team_2_odds_html = team_2_odds_tr.select(&data_moneyline_selector).next();


                if !team_1_odds_html.is_some() && !team_2_odds_html.is_some() {
                    continue;
                }

                let team_1_odds_inner_html = team_1_odds_html.unwrap().inner_html();
                let team_1_odds = team_1_odds_inner_html.trim();
                let team_2_odds_inner_html = team_2_odds_html.unwrap().inner_html();
                let team_2_odds = team_2_odds_inner_html.trim();


                let q = "
                insert into vegas.lines (game_time, team_1_id, team_1_name, team_1_abbreviation, team_2_id, team_2_name, team_2_abbreviation, sportsbook, line_type, 
                    team_1_handicap, team_1_odds, team_2_handicap, team_2_odds)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                on conflict (team_1_id, team_2_id, sportsbook, game_time, line_type) do update set 
                team_1_name = excluded.team_1_name,
                team_1_abbreviation = excluded.team_1_abbreviation,
                team_2_name = excluded.team_2_name,
                team_2_abbreviation = excluded.team_2_abbreviation,
                team_1_handicap = excluded.team_1_handicap,
                team_1_odds = excluded.team_1_odds,
                team_2_handicap = excluded.team_2_handicap,
                team_2_odds = excluded.team_2_odds;
                ";

                let result = sqlx::query(q)
                .bind(game_time.to_string())
                .bind(team_1_id)
                .bind(team_1_name.to_string())
                .bind(team_1_abbreviation.to_string())
                .bind(team_2_id)
                .bind(team_2_name.to_string())
                .bind(team_2_abbreviation.to_string())
                .bind(sportsbook.to_string())
                .bind(line_type)
                .bind(Option::<f32>::None)
                .bind(team_1_odds.parse::<i32>().unwrap_or(-100))
                .bind(Option::<f32>::None)
                .bind(team_2_odds.parse::<i32>().unwrap_or(-100))
                .execute(&state.pool);

                v.push(result);

            }

        };
        
    }


    let stream = futures::stream::iter(v).buffer_unordered(10);
    let results = stream.collect::<Vec<_>>().await;

    for r in results {
        if r.is_err() {
            println!("{:?}", r);
        }
    }

    "Success"
}