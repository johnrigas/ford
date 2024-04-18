use axum::{extract::State, Json};
use axum_macros::debug_handler;
use futures::StreamExt;
use reqwest::Method;
use serde::Deserialize;
use crate::utils::AppState;
use scraper::{Html, Selector};
use crate::refresh::utils;
use reqwest::{cookie::Jar, Url};
use std::future::Future;
use sqlx::postgres::PgQueryResult;


#[derive(Debug, Deserialize)]
pub struct KenpomRefreshPayload {
    pub team_rankings_year: Option<String>,
    pub game_start_date: Option<String>,
    pub game_end_date: Option<String>,
    pub session_id_cookie: String
}


#[debug_handler]
pub async fn refresh_kenpom(State(state): State<AppState>, Json(payload): Json<KenpomRefreshPayload>) -> &'static str {
    #[derive(sqlx::FromRow, Debug)]
    struct Date { day: String }

    let mut v = Vec::new(); 

    let token = payload.session_id_cookie;

    if let (Some(start), Some(end)) = (payload.game_start_date, payload.game_end_date) {

        let q = &format!("
        SELECT date_trunc('day', dd)::date::varchar as day
        FROM generate_series
                ( '{start}'::timestamp
                , '{end}'::timestamp
                , '1 day'::interval) dd
                ;
        ");
    
        let result = sqlx::query_as::<_, Date>(q).fetch_all(&state.pool).await.unwrap();
        let mut days = Vec::new();

        let mut fanmatch_responses = Vec::new();
        for day in result {
            days.push(day.day.clone());
            let today = day.day;
            let cookie = &format!("PHPSESSID={token}; Domain=kenpom.com");
            let fanmatch_url = format!("https://kenpom.com/fanmatch.php?d={today}").parse::<Url>().unwrap();
            let jar = Jar::default();
            jar.add_cookie_str(cookie, &fanmatch_url);
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("User-agent", "Mozilla/5.0".parse().unwrap());
            let fanmatch_resp = utils::api(fanmatch_url, Method::GET, headers, vec![], {}, Some(jar)).await.text().await.unwrap();
            fanmatch_responses.push(fanmatch_resp)
        };


        'days: for (idx,fanmatch_resp) in fanmatch_responses.iter().enumerate() {
            let fanmatch_document = Html::parse_document(&fanmatch_resp);
            let tb_selector = Selector::parse("#fanmatch-table > tbody").unwrap();
            // println!("{:?}", fanmatch_document);

            let tb = fanmatch_document.select(&tb_selector).next().unwrap();
            let tr_selector = Selector::parse("tr").unwrap();
            let trs = tb.select(&tr_selector);
            let td_selector = Selector::parse("td").unwrap();
            let a_selector = Selector::parse("a").unwrap();
            let span_selector = Selector::parse("span").unwrap();
    
            for tr in trs {

                let tr_html = tr.inner_html();
                if tr_html.contains("o' the night") {
                    continue 'days;
                }
    
                let mut tds = tr.select(&td_selector);
                let td1 = tds.next().unwrap();
                let mut spans = td1.select(&span_selector);
                let mut a_s = td1.select(&a_selector);

                let team_1_rank = match spans.next().unwrap().inner_html().as_str() {
                    "" => "NR".to_string(),
                    other => other.to_string()
                };
                spans.next();
                let team_2_rank = match spans.next().unwrap().inner_html().as_str() {
                    "" => "NR".to_string(),
                    other => other.to_string()
                };
    
                let first_a_tag = a_s.next();
                let second_a_tag = a_s.next();
    
                let (team_1, team_2) = match (first_a_tag, second_a_tag, team_1_rank.as_str(), team_2_rank.as_str()) {
                    (_, _, "NR", "NR") =>  (Some("Non D1 School".to_string()),  Some("Non D1 School".to_string())),
                    (Some(a_tag), _, _, "NR") => (Some(a_tag.inner_html()),  Some("Non D1 School".to_string())),
                    (Some(a_tag), _, "NR", _) => (Some("Non D1 School".to_string()), Some(a_tag.inner_html())),
                    (_, _, _, _) => (Some(first_a_tag.unwrap().inner_html()), Some(second_a_tag.unwrap().inner_html()))
                };
    
                let projection = tds.next().unwrap().inner_html();
                let (team_1_projected_score, team_2_projected_score, team_1_projection, team_2_projection) = if team_1_rank != "NR" && team_2_rank != "NR" {
    
                    let mut projection_split = projection.rsplit(" ");
                    let percent_full_string = projection_split.next().unwrap();
                    // println!("{:?}", percent_full_string);
                    // println!("{:?}", team_1_rank);
                    // println!("{:?}", team_2_rank);
                    // println!("{:?}", team_1);
                    // println!("{:?}", team_2);
                    let projected_percent_string = &percent_full_string[1..3];
                    let projected_score = &projection_split.next().unwrap();
                    // let projected_winner = &projection_split.next().unwrap();
                    let mut projected_winner_words: Vec<&str> = projection_split.collect();
                    projected_winner_words.reverse();
                    let projected_winner = projected_winner_words.join(" ");
                    let projected_percent = projected_percent_string.parse::<i32>().unwrap();
                    // println!("{:?}", projected_winner);
        
                    let mut projected_score_split = projected_score.split("-");
                    let winning_score = projected_score_split.next().unwrap();
                    let losing_score = projected_score_split.next().unwrap();
        
                    let team_1_projected_score = if team_1 == Some(projected_winner.to_string()) {winning_score} else {losing_score};
                    let team_2_projected_score = if team_2 == Some(projected_winner.to_string()) {winning_score} else {losing_score};
                    let team_1_projection = if team_1 == Some(projected_winner.to_string()) {(projected_percent as f64)/(100.0 as f64)} else {(1.0 as f64) - (projected_percent as f64) / (100.0 as f64) };
                    let team_2_projection = if team_2 == Some(projected_winner.to_string()) {(projected_percent as f64)/(100.0 as f64)} else {(1.0 as f64) - (projected_percent as f64) / (100.0 as f64) };
                    (Some(team_1_projected_score), Some(team_2_projected_score), Some(team_1_projection), Some(team_2_projection))
                } else {
                    (Option::<&str>::None, Option::<&str>::None, Option::<f64>::None, Option::<f64>::None)
                };
    
                tds.next();
    
                let location_td = tds.next().unwrap();
                let game_venue = location_td.select(&a_selector).next().unwrap().select(&span_selector).next().unwrap().inner_html();
                // let location_td_html = location_td.inner_html();
                // let mut location_td_split = location_td_html.split("\"");
                // location_td_split.next();
                // let game_location = location_td_split.next().unwrap();
                let game_location = "";
                let thrill_score = 0.0;
                let comeback = 0.0;
                let excitement = 0.0;

                let team_1_revised = if team_1 > team_2 {team_1.clone()} else {team_2.clone()};
                let team_1_rank_revised = if team_1 > team_2 {team_1_rank.clone()} else {team_2_rank.clone()};
                let team_1_projected_score_revised = if team_1 > team_2 {team_1_projected_score.clone()} else {team_2_projected_score.clone()};
                let team_1_projection_revised = if &team_1 > &team_2 {team_1_projection.clone()} else {team_2_projection.clone()};

                let team_2_revised = if team_1 <= team_2 {team_1.clone()} else {team_2.clone()};
                let team_2_rank_revised = if team_1 <= team_2 {team_1_rank.clone()} else {team_2_rank.clone()};
                let team_2_projected_score_revised = if team_1 <= team_2 {team_1_projected_score.clone()} else {team_2_projected_score.clone()};
                let team_2_projection_revised = if team_1 <= team_2 {team_1_projection.clone()} else {team_2_projection.clone()};
    
                let q = "
                insert into kenpom.fanmatch_games (game_date, team_1, team_1_rank, team_1_projected_score, team_1_projection, 
                    team_2, team_2_rank, team_2_projected_score, team_2_projection, game_location, 
                    game_venue, thrill_score, comeback, excitement)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                on conflict (game_date, team_1, team_2) do update set 
                team_1_rank = excluded.team_1_rank,
                team_1_projected_score = excluded.team_1_projected_score,
                team_1_projection = excluded.team_1_projection,
                team_2_rank = excluded.team_2_rank,
                team_2_projected_score = excluded.team_2_projected_score,
                team_2_projection = excluded.team_2_projection,
                game_location = excluded.game_location,
                game_venue = excluded.game_venue,
                thrill_score = excluded.thrill_score,
                comeback = excluded.comeback,
                excitement = excluded.excitement;
                ";
        
                let result = sqlx::query(q)
                .bind(days[idx].clone())
                .bind(team_1_revised)
                .bind(team_1_rank_revised.parse::<i32>().unwrap_or(0))
                .bind(team_1_projected_score_revised.unwrap_or(&"0").parse::<i32>().unwrap())
                .bind(team_1_projection_revised)
                .bind(team_2_revised)
                .bind(team_2_rank_revised.parse::<i32>().unwrap_or(0))
                .bind(team_2_projected_score_revised.unwrap_or(&"0").parse::<i32>().unwrap())
                .bind(team_2_projection_revised)
                .bind(game_location)
                .bind(game_venue)
                .bind(thrill_score)
                .bind(comeback)
                .bind(excitement)
                .execute(&state.pool);
    
                v.push(result);
            }

        }

    };

    
    if let Some(y) = payload.team_rankings_year {
            
        let home_url = format!("https://kenpom.com/index.php?y={y}");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-agent", "Mozilla/5.0".parse().unwrap());
        let home_resp = utils::api(home_url, Method::GET, headers, vec![], {}, Option::<Jar>::None).await.text().await.unwrap();

        let home_document = Html::parse_document(&home_resp);

        let tb_selector = Selector::parse("#ratings-table > tbody").unwrap();
        let tbs = home_document.select(&tb_selector);
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let a_selector = Selector::parse("a").unwrap();
        // let span_selector = Selector::parse("span").unwrap();

        for tb in tbs {
            let trs = tb.select(&tr_selector);

            for tr in trs {
                let mut tds = tr.select(&td_selector);
                tds.next();
                let team = tds.next().unwrap().select(&a_selector).next().unwrap().inner_html();
                let conference = tds.next().unwrap().select(&a_selector).next().unwrap().inner_html();
                let record = tds.next().unwrap().inner_html();

                let mut record_split = record.split("-");
                let wins = &record_split.next().unwrap();
                let losses = &record_split.next().unwrap();

                let adj_em = tds.next().unwrap().inner_html();
                let adj_o = tds.next().unwrap().inner_html();
                tds.next();
                let adj_d = tds.next().unwrap().inner_html();
                tds.next();
                let adj_t = tds.next().unwrap().inner_html();
                tds.next();
                let luck = tds.next().unwrap().inner_html();
                tds.next();
                let sos_adj_em = tds.next().unwrap().inner_html();
                tds.next();
                let sos_adj_o = tds.next().unwrap().inner_html();
                tds.next();
                let sos_adj_d = tds.next().unwrap().inner_html();
                tds.next();
                let nc_sos_adj_em = tds.next().unwrap().inner_html();
                tds.next();

                let q = "
                insert into kenpom.teams (team, conference, wins, losses, adj_em, adj_o, adj_d, adj_t, luck, sos_adj_em, sos_adj_o, sos_adj_d, nc_sos_adj_em)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                on conflict (team) do update set 
                conference = excluded.conference,
                wins = excluded.wins,
                losses = excluded.losses,
                adj_em = excluded.adj_em,
                adj_o = excluded.adj_o,
                adj_d = excluded.adj_d,
                adj_t = excluded.adj_t,
                luck = excluded.luck,
                sos_adj_em = excluded.sos_adj_em,
                sos_adj_o = excluded.sos_adj_o,
                sos_adj_d = excluded.sos_adj_d,
                nc_sos_adj_em = excluded.nc_sos_adj_em;
                ";
        
                let result = sqlx::query(q)
                .bind(team)
                .bind(conference)
                .bind(wins.parse::<i32>().unwrap())
                .bind(losses.parse::<i32>().unwrap())
                .bind(adj_em.parse::<f32>().unwrap())
                .bind(adj_o.parse::<f32>().unwrap())
                .bind(adj_d.parse::<f32>().unwrap())
                .bind(adj_t.parse::<f32>().unwrap())
                .bind(luck.parse::<f32>().unwrap())
                .bind(sos_adj_em.parse::<f32>().unwrap())
                .bind(sos_adj_o.parse::<f32>().unwrap())
                .bind(sos_adj_d.parse::<f32>().unwrap())
                .bind(nc_sos_adj_em.parse::<f32>().unwrap())
                .execute(&state.pool);

                v.push(result);

            }
        }



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