use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use crate::utils::AppState;
use scraper::{Html, Selector};
use crate::refresh::utils;
use serde_json::Value;
use std::collections::HashMap;
use reqwest::{Method, cookie::Jar};
use std::{thread, time};


#[derive(Deserialize, Debug)]
struct Header {

}

#[derive(Deserialize, Debug)]
struct PredictorTeam {
    gameProjection: f64
}

#[derive(Deserialize, Debug)]
struct Predictor {
    homeTeam: PredictorTeam,
    awayTeam: PredictorTeam

}

#[derive(Deserialize, Debug)]
struct OddsDisplay {
    alternateDisplayValue: String
}


#[derive(Deserialize, Debug)]
struct TotalOdds {
    total: OddsDisplay,
    over: OddsDisplay,
    under: OddsDisplay
}


#[derive(Deserialize, Debug)]
struct Pickcenter {
    open: TotalOdds,
    current: TotalOdds
}

#[derive(Deserialize, Debug)]
struct Boxscore {

}

#[derive(Deserialize, Debug)]
struct GameInfo {
    
}

#[derive(Deserialize, Debug)]
struct Plays {

}


#[derive(Deserialize, Debug)]
struct Summary {
    predictor: Predictor,
    pickcenter: Pickcenter,
    boxscore: Boxscore,
    header: Header,
    gameInfo: GameInfo,
    plays: Plays
}

#[derive(Debug, Deserialize)]
pub struct ESPNRefreshPayload {
    pub game_start_date: Option<String>,
    pub game_end_date: Option<String>
}


#[debug_handler]
pub async fn refresh_espn(State(state): State<AppState>, Json(payload): Json<ESPNRefreshPayload>) -> &'static str {
    #[derive(sqlx::FromRow, Debug)]
    struct Date { day: String }

    if let (Some(start), Some(end)) = (payload.game_start_date, payload.game_end_date) {

        let q = &format!("
        SELECT replace(date_trunc('day', dd):: date::varchar, '-', '') as day
        FROM generate_series
                ( '{start}'::timestamp
                , '{end}'::timestamp
                , '1 day'::interval) dd
                ;
        ");
    
        let result = sqlx::query_as::<_, Date>(q).fetch_all(&state.pool).await.unwrap();
    
        for day in result.iter() {
    
            println!("Running {:?}", day.day);
    
            let day_string = &day.day;
    
            let game_ids: Vec<String> = {
                let url = format!("https://www.espn.com/mens-college-basketball/scoreboard/_/date/{day_string}/group/50");
                let resp = utils::api(url, Method::GET, reqwest::header::HeaderMap::new(), vec![], {},  Option::<Jar>::None).await.text().await.unwrap();
                let document = Html::parse_document(&resp);
                let selector = Selector::parse(".gameModules > div > section").unwrap();
                document.select(&selector).map(|game| game.attr("id").unwrap().to_string()).collect()
            };
            // let _gs = &*game_ids;
            let i = 5;
            let j = &i;
            let k = *j;

    
            let urls = game_ids.into_iter().map(|game_id| format!("https://site.web.api.espn.com/apis/site/v2/sports/basketball/mens-college-basketball/summary?region=us&lang=en&contentorigin=espn&event={game_id}&showAirings=buy%2Clive%2Creplay&showZipLookup=true&buyWindow=1m")).collect();
            let responses = utils::fetch_many(urls).await;
            for response in responses {
                let b: Value = serde_json::from_str(&response).unwrap();
    
                let predictor = &b["predictor"];
                let pickcenter_array = &b["pickcenter"].as_array();
    
                let q = "
                insert into espn.games (game_id, over_under, over_odds, under_odds, opening_over_under, opening_over_odds, opening_under_odds, start_time)
                values ($1, $2, $3, $4, $5, $6, $7, $8)
                on conflict (game_id) do update set 
                over_under = excluded.over_under,
                over_odds = excluded.over_odds,
                under_odds = excluded.under_odds,
                opening_over_under = excluded.opening_over_under,
                opening_over_odds = excluded.opening_over_odds,
                opening_under_odds = excluded.opening_under_odds,
                start_time = excluded.start_time;
                ";
    
                if pickcenter_array.is_some() {
                    let pickcenter = pickcenter_array.unwrap().first();
    
                    if pickcenter.is_some() {
                        let _result = sqlx::query(q)
                        .bind(&b["header"]["id"].as_str())
                        .bind(&pickcenter.unwrap()["current"]["total"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<f32>().unwrap())))
                        .bind(&pickcenter.unwrap()["current"]["over"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
                        .bind(&pickcenter.unwrap()["current"]["under"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
                        .bind(&pickcenter.unwrap()["open"]["total"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<f32>().unwrap())))
                        .bind(&pickcenter.unwrap()["open"]["over"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
                        .bind(&pickcenter.unwrap()["open"]["under"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
                        .bind(&day.day)
                        .execute(&state.pool).await.unwrap();
    
                    } else {
                        let _result = sqlx::query(q)
                        .bind(&b["header"]["id"].as_str())
                        .bind(Option::<f32>::None)
                        .bind(Option::<i32>::None)
                        .bind(Option::<i32>::None)
                        .bind(Option::<f32>::None)
                        .bind(Option::<i32>::None)
                        .bind(Option::<i32>::None)
                        .bind(&day.day)
                        .execute(&state.pool).await.unwrap();
                    }
    
    
                } else {
                    println!("header {:?}", &b["header"]);

                    let _result = sqlx::query(q)
                    .bind(&b["header"]["id"].as_str())
                    .bind(Option::<f32>::None)
                    .bind(Option::<i32>::None)
                    .bind(Option::<i32>::None)
                    .bind(Option::<f32>::None)
                    .bind(Option::<i32>::None)
                    .bind(Option::<i32>::None)
                    .bind(&day.day)
                    .execute(&state.pool).await.unwrap();
    
                }
    
                let boxscore = &b["boxscore"];
                for team in boxscore["teams"].as_array().unwrap() {
                    let t = &team["team"];
    
                    let q = "
                        insert into espn.teams (team_id, uid, slug, location, name, abbreviation, display_name, short_display_name, color, logo)
                        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                        on conflict (team_id) do nothing;
                    ";
    
                    let _result = sqlx::query(q)
                    .bind(&t["id"].as_str())
                    .bind(&t["uid"].as_str())
                    .bind(&t["slug"].as_str())
                    .bind(&t["location"].as_str())
                    .bind(&t["name"].as_str())
                    .bind(&t["abbreviation"].as_str())
                    .bind(&t["displayName"].as_str())
                    .bind(&t["shortDisplayName"].as_str())
                    .bind(&t["color"].as_str())
                    .bind(&t["logo"].as_str())
                    .execute(&state.pool).await.unwrap();
    
    
                    let s: HashMap<_, _> = team["statistics"].as_array().unwrap().into_iter().map(|stat| (stat["name"].as_str().unwrap(), stat["displayValue"].as_str().unwrap())).collect();
    
                    let mut fg = s.get("fieldGoalsMade-fieldGoalsAttempted").unwrap_or(&"0-0").split("-");
                    let fgm = &fg.next().unwrap();
                    let fga = &fg.next().unwrap();
    
                    let mut _3p = s.get("threePointFieldGoalsMade-threePointFieldGoalsAttempted").unwrap_or(&"0-0").split("-");
                    let _3pm = &_3p.next().unwrap();
                    let _3pa = &_3p.next().unwrap();
    
                    let mut ft = s.get("freeThrowsMade-freeThrowsAttempted").unwrap_or(&"0-0").split("-");
                    let ftm = &ft.next().unwrap();
                    let fta = &ft.next().unwrap();
    
                    let home = if &predictor["homeTeam"]["id"].as_str() == &t["id"].as_str() {true} else {false};
                    let game_projection = if home {&predictor["homeTeam"]["gameProjection"]} else {&predictor["awayTeam"]["gameProjection"]};
    
    
                    let q = "
                    insert into espn.game_teams (team_id, game_id, fgm, fga, _3pm, _3pa, ftm, fta, reb, oreb, assists, steals, blocks, turnovers, techs, flagrants, turnover_points,
                        fast_break_points, points_in_paint, fouls, largest_lead, game_projection, home, favorite, underdog, moneyline, spread, spread_odds,
                        opening_moneyline, opening_spread, opening_spread_odds
                    )
                    values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31)
                    on conflict (team_id, game_id) do update
                    set fgm = excluded.fgm, 
                        fga = excluded.fga, 
                        _3pm = excluded._3pm, 
                        _3pa = excluded._3pa,  
                        ftm = excluded.ftm, 
                        fta = excluded.fta, 
                        reb = excluded.reb, 
                        oreb = excluded.oreb, 
                        assists = excluded.assists, 
                        steals = excluded.steals, 
                        blocks = excluded.blocks, 
                        turnovers = excluded.turnovers, 
                        techs = excluded.techs, 
                        flagrants = excluded.flagrants, 
                        turnover_points = excluded.turnover_points, 
                        fast_break_points = excluded.fast_break_points, 
                        points_in_paint = excluded.points_in_paint, 
                        fouls = excluded.fouls, 
                        largest_lead = excluded.largest_lead,
                        game_projection = excluded.game_projection, 
                        home = excluded.home,
                        favorite = excluded.favorite, 
                        underdog = excluded.underdog, 
                        moneyline = excluded.moneyline, 
                        spread = excluded.spread, 
                        spread_odds = excluded.spread_odds, 
                        opening_moneyline = excluded.opening_moneyline, 
                        opening_spread = excluded.opening_spread,
                        opening_spread_odds = excluded.opening_spread_odds;
                    ";
    
    
                    if pickcenter_array.is_some() { 
                        let pickcenter = pickcenter_array.unwrap().first();
    
                        if pickcenter.is_some() {
                            let odds = if home {&pickcenter.unwrap()["homeTeamOdds"]} else {&pickcenter.unwrap()["awayTeamOdds"]};
        
                            let _result = sqlx::query(q)
                            .bind(&t["id"].as_str())
                            .bind(&b["header"]["id"].as_str())
                            .bind(fgm.parse::<i32>().unwrap())
                            .bind(fga.parse::<i32>().unwrap())
                            .bind(_3pm.parse::<i32>().unwrap())
                            .bind(_3pa.parse::<i32>().unwrap())
                            .bind(ftm.parse::<i32>().unwrap())
                            .bind(fta.parse::<i32>().unwrap())
                            .bind(s.get("totalRebounds").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("offensiveRebounds").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("assists").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("steals").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("blocks").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("turnovers").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("technicalFouls").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("flagrantFouls").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("turnoverPoints").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("fastBreakPoints").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("pointsInPaint").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("fouls").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("largestLead").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(game_projection.as_str().and_then(|v| Some(v.parse::<f32>().unwrap())))
                            .bind(home)
                            .bind(odds["favorite"].as_bool())
                            .bind(odds["underdog"].as_bool())
                            .bind(odds["current"]["moneyLine"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
                            .bind(odds["current"]["pointSpread"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<f32>().unwrap_or(0.0))))
                            .bind(odds["current"]["spread"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
                            .bind(odds["open"]["moneyLine"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
                            .bind(odds["open"]["pointSpread"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<f32>().unwrap_or(0.0))))
                            .bind(odds["open"]["spread"]["alternateDisplayValue"].as_str().and_then(|v| Some(v.parse::<i32>().unwrap_or(100))))
                            .execute(&state.pool).await.unwrap();
        
                        } else {
                            let _result = sqlx::query(q)
                            .bind(&t["id"].as_str())
                            .bind(&b["header"]["id"].as_str())
                            .bind(fgm.parse::<i32>().unwrap())
                            .bind(fga.parse::<i32>().unwrap())
                            .bind(_3pm.parse::<i32>().unwrap())
                            .bind(_3pa.parse::<i32>().unwrap())
                            .bind(ftm.parse::<i32>().unwrap())
                            .bind(fta.parse::<i32>().unwrap())
                            .bind(s.get("totalRebounds").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("offensiveRebounds").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("assists").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("steals").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("blocks").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("turnovers").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("technicalFouls").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("flagrantFouls").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("turnoverPoints").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("fastBreakPoints").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("pointsInPaint").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("fouls").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(s.get("largestLead").unwrap_or(&"0").parse::<i32>().unwrap())
                            .bind(game_projection.as_str().and_then(|v| Some(v.parse::<f32>().unwrap())))
                            .bind(home)
                            .bind(Option::<bool>::None)
                            .bind(Option::<bool>::None)
                            .bind(Option::<i32>::None)
                            .bind(Option::<f32>::None)
                            .bind(Option::<i32>::None)
                            .bind(Option::<i32>::None)
                            .bind(Option::<f32>::None)
                            .bind(Option::<i32>::None)
                            .execute(&state.pool).await.unwrap();
        
                        }
                    } else {
                        let _result = sqlx::query(q)
                        .bind(&t["id"].as_str())
                        .bind(&b["header"]["id"].as_str())
                        .bind(fgm.parse::<i32>().unwrap())
                        .bind(fga.parse::<i32>().unwrap())
                        .bind(_3pm.parse::<i32>().unwrap())
                        .bind(_3pa.parse::<i32>().unwrap())
                        .bind(ftm.parse::<i32>().unwrap())
                        .bind(fta.parse::<i32>().unwrap())
                        .bind(s.get("totalRebounds").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("offensiveRebounds").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("assists").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("steals").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("blocks").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("turnovers").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("technicalFouls").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("flagrantFouls").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("turnoverPoints").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("fastBreakPoints").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("pointsInPaint").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("fouls").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(s.get("largestLead").unwrap_or(&"0").parse::<i32>().unwrap())
                        .bind(game_projection.as_str().and_then(|v| Some(v.parse::<f32>().unwrap())))
                        .bind(home)
                        .bind(Option::<bool>::None)
                        .bind(Option::<bool>::None)
                        .bind(Option::<i32>::None)
                        .bind(Option::<f32>::None)
                        .bind(Option::<i32>::None)
                        .bind(Option::<i32>::None)
                        .bind(Option::<f32>::None)
                        .bind(Option::<i32>::None)
                        .execute(&state.pool).await.unwrap();
    
                    }
    
    
    
                }
    
                let info = &b["gameInfo"];
                for official in info["officials"].as_array().unwrap_or(&Vec::<Value>::new()) {
    
                    let q = "
                    insert into espn.officials (name)
                    values ($1)
                    on conflict (name) do nothing;
                    ";
            
                    let _result = sqlx::query(q)
                    .bind(&official["fullName"].as_str())
                    .execute(&state.pool).await.unwrap();
            
                    let q = "
                    insert into espn.game_officials (name, game_id)
                    values ($1, $2)
                    on conflict (name, game_id) do nothing;
                    ";
            
                    let _result = sqlx::query(q)
                    .bind(&official["fullName"].as_str())
                    .bind(&b["header"]["id"].as_str())
                    .execute(&state.pool).await.unwrap();
            
                    // println!("{:?}", _result);
    
                }
    
                let venue = &info["venue"];
    
                if venue["id"].as_str().is_some() {
    
                    let q = "
                    insert into espn.venues (venue_id, name, city, state, capacity)
                    values ($1, $2, $3, $4, $5)
                    on conflict (venue_id) do nothing;
                    ";
    
    
                    let _result = sqlx::query(q)
                    .bind(&venue["id"].as_str())
                    .bind(&venue["fullName"].as_str())
                    .bind(&venue["address"]["city"].as_str())
                    .bind(&venue["address"]["state"].as_str())
                    .bind(&venue["capacity"].as_i64())
                    .execute(&state.pool).await.unwrap();
    
                    let q = "
                    insert into espn.game_venues (venue_id, game_id, attendance)
                    values ($1, $2, $3)
                    on conflict (venue_id, game_id) do update
                    set attendance = excluded.attendance;
                    ";
    
                    let _result = sqlx::query(q)
                    .bind(&venue["id"].as_str())
                    .bind(&b["header"]["id"].as_str())
                    .bind(&info["attendance"].as_i64())
                    .execute(&state.pool).await.unwrap();
                        
                }
    
    
                for team in boxscore["players"].as_array().unwrap_or(&Vec::<Value>::new()) {
                    let team_id = &team["team"]["id"];
                    let players = &team["statistics"].as_array().unwrap().first().unwrap()["athletes"].as_array().unwrap();
                    for player in players.into_iter().filter(|p| p["stats"].as_array().unwrap().len() > 0) {
    
                        let athlete = &player["athlete"];
                        let stats = &player["stats"].as_array().unwrap();
    
                        let q = "
                        insert into espn.players (player_id, uid, guid, display_name, short_name, position)
                        values ($1, $2, $3, $4, $5, $6)
                        on conflict (player_id) do nothing;
                        ";
    
                
                        let _result = sqlx::query(q)
                        .bind(&athlete["id"].as_str())
                        .bind(&athlete["uid"].as_str())
                        .bind(&athlete["guid"].as_str())
                        .bind(&athlete["displayName"].as_str())
                        .bind(&athlete["shortName"].as_str())
                        .bind(&athlete["position"]["name"].as_str())
                        .execute(&state.pool).await.unwrap();
    
                        let q = "
                        insert into espn.game_players (player_id, game_id, team_id, active, started, played, ejected, min, fgm, fga, _3pm, _3pa, ftm, fta, reb, oreb,
                            assists, steals, blocks, turnovers, fouls, points)
                        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
                        on conflict (player_id, game_id, team_id) do update set 
                        active = excluded.active,
                        started = excluded.started,
                        played = excluded.played,
                        ejected = excluded.ejected,
                        min = excluded.min,
                        fgm = excluded.fgm,
                        fga = excluded.fga,
                        _3pm = excluded._3pm,
                        _3pa = excluded._3pa,
                        ftm = excluded.ftm,
                        fta = excluded.fta,
                        reb = excluded.reb,
                        oreb = excluded.oreb,
                        assists = excluded.assists,
                        steals = excluded.steals,
                        turnovers = excluded.turnovers,
                        fouls = excluded.fouls,
                        blocks = excluded.blocks,
                        points = excluded.points;
                        ";
    
                        let mut fg = stats[1].as_str().unwrap().split("-");
                        let fgm = &fg.next().unwrap();
                        let fga = &fg.next().unwrap();
            
                        let mut _3p = stats[2].as_str().unwrap().split("-");
                        let _3pm = &_3p.next().unwrap();
                        let _3pa = &_3p.next().unwrap();
            
                        let mut ft = stats[3].as_str().unwrap().split("-");
                        let ftm = &ft.next().unwrap();
                        let fta = &ft.next().unwrap();
    
                        // println!("{:?}", stats);
    
                        let _result = sqlx::query(q)
                        .bind(&athlete["id"].as_str())
                        .bind(&b["header"]["id"].as_str())
                        .bind(team_id.as_str())
                        .bind(&player["active"].as_bool())
                        .bind(&player["starter"].as_bool())
                        .bind(&player["didNotPlay"].as_bool().unwrap())
                        .bind(&player["ejected"].as_bool())
                        .bind(&stats[0].as_str().unwrap().parse::<i32>().unwrap_or(0))
                        .bind(fgm.parse::<i32>().unwrap())
                        .bind(fga.parse::<i32>().unwrap())
                        .bind(_3pm.parse::<i32>().unwrap())
                        .bind(_3pa.parse::<i32>().unwrap())
                        .bind(ftm.parse::<i32>().unwrap())
                        .bind(fta.parse::<i32>().unwrap())
                        .bind(&stats[6].as_str().unwrap().parse::<i32>().unwrap())
                        .bind(&stats[4].as_str().unwrap().parse::<i32>().unwrap())
                        .bind(&stats[7].as_str().unwrap().parse::<i32>().unwrap())
                        .bind(&stats[8].as_str().unwrap().parse::<i32>().unwrap())
                        .bind(&stats[9].as_str().unwrap().parse::<i32>().unwrap())
                        .bind(&stats[10].as_str().unwrap().parse::<i32>().unwrap())
                        .bind(&stats[11].as_str().unwrap().parse::<i32>().unwrap())
                        .bind(&stats[12].as_str().unwrap().parse::<i32>().unwrap())
                        .execute(&state.pool).await.unwrap();
    
                    }
    
                }
    
                let plays = &b["plays"];
    
                for play in plays.as_array().unwrap_or(&Vec::<Value>::new()).into_iter() {
                    let q = "
                    insert into espn.plays (play_id, game_id, sequence_number, play_type, play_text, away_score, home_score,
                        period, clock_minutes, clock_seconds, scoring_play, score_value, team_id, wall_clock, shooting_play)
                    values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                    on conflict (play_id) do nothing;
                    ";
    
                    let mut clock = play["clock"]["displayValue"].as_str().unwrap().split(':');
                    let clock_minutes = &clock.next().unwrap().parse::<i32>().unwrap();
                    let clock_seconds = &clock.next().unwrap().parse::<i32>().unwrap();
    
                    let _result = sqlx::query(q)
                    .bind(&play["id"].as_str())
                    .bind(&b["header"]["id"].as_str())
                    .bind(&play["sequenceNumber"].as_str())
                    .bind(&play["type"]["text"].as_str())
                    .bind(&play["text"].as_str())
                    .bind(&play["awayScore"].as_i64())
                    .bind(&play["homeScore"].as_i64())
                    .bind(&play["period"]["number"].as_i64())
                    .bind(clock_minutes)
                    .bind(clock_seconds)
                    .bind(&play["scoringPlay"].as_bool())
                    .bind(&play["scoreValue"].as_i64())
                    .bind(&play["team"]["id"].as_str())
                    .bind(&play["wallclock"].as_str())
                    .bind(&play["shootingPlay"].as_bool())
                    .execute(&state.pool).await.unwrap();
                    
                    if play["participants"].is_array() {
                        for player in play["participants"].as_array().unwrap().into_iter() {
                            let q = "
                            insert into espn.play_participants (play_id, player_id)
                            values ($1, $2)
                            on conflict (play_id, player_id) do nothing;
                            ";
                
                            let _result = sqlx::query(q)
                            .bind(&play["id"].as_str())
                            .bind(&player["athlete"]["id"].as_str())
                            .execute(&state.pool).await.unwrap();
    
                        }
                    }
    
    
    
                }
            }


            let t = time::Duration::from_millis(200000);
            println!("Sleeping");
            thread::sleep(t);
            println!("Done sleeping");
    
        }
         

    }



    "Success"
}





// -- select g.game_id, g.over_under::float8, g.over_odds, g.under_odds,
// --        g.opening_over_under::float8, g.opening_over_odds, g.opening_under_odds,
// --        t.display_name, gt.home, gt.moneyline, gt.opening_moneyline,
// --        gt.game_projection::float, gt.spread::float, gt.spread_odds,
// --        gt.opening_spread::float, gt.opening_spread_odds
// --     from espn.games g
// --     join espn.game_teams gt on g.game_id = gt.game_id
// --     join espn.teams t on t.team_id = gt.team_id
// -- where start_time = '20231219'
// -- order by g.game_id;  

// select game, source, away_projection, home_projection,
//   away_moneyline, home_moneyline,
//   round(case when away_moneyline > 0 then 100.0/(away_moneyline + 100) end as away_ml_implied_prob, 3),
//   round(case when home_moneyline > 0 then 100.0/(home_moneyline + 100) end as home_ml_implied_prob, 3)
// from summary.games
// where game ilike '%20231220%'
// order by game, source
// ;  