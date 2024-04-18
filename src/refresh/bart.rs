use axum::{extract::State, Json};
use axum_macros::debug_handler;
use futures::StreamExt;
use reqwest::Method;
use serde::{Deserialize, Deserializer};
use crate::utils::AppState;
use scraper::{Html, Selector};
use crate::refresh::utils;
use reqwest::{cookie::Jar, Url};


pub fn deserialize_option_string_from_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumberOrNull {
        String(String),
        Number(i64),
        Float(f64),
        Null
    }

    match StringOrNumberOrNull::deserialize(deserializer)? {
        StringOrNumberOrNull::String(s) => Ok(Some(s)),
        StringOrNumberOrNull::Number(i) => Ok(Some(i.to_string())),
        StringOrNumberOrNull::Float(f) => Ok(Some(f.to_string())),
        StringOrNumberOrNull::Null => Ok(None),
    }
}

#[derive(Deserialize, Debug)]
struct Player (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<i64>,
    Option<i64>,
    Option<f64>,
    Option<i64>,
    Option<i64>,
    Option<f64>,
    Option<i64>,
    Option<i64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<String>,
    Option<String>,
    #[serde(deserialize_with = "deserialize_option_string_from_number")]
    Option<String>, //num
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<i64>,
    Option<i64>,
    Option<String>,
    Option<f64>,
    Option<f64>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<i64>,
    Option<i64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<String>,
    Option<f64>
);


#[derive(Debug, Deserialize)]
pub struct BartRefreshPayload {
    pub player_stats_year: Option<String>,
    pub team_rankings_year: Option<String>,
    pub game_start_date: Option<String>,
    pub game_end_date: Option<String>
}


#[debug_handler]
pub async fn refresh_bart(State(state): State<AppState>, Json(payload): Json<BartRefreshPayload>) -> &'static str {

    let mut v = Vec::new(); 

    if let Some(y) = payload.player_stats_year {

        let home_url = format!("https://barttorvik.com/getadvstats.php?year={y}");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-agent", "Mozilla/5.0".parse().unwrap());
        let resp = utils::api(home_url, Method::GET, headers, vec![], {}, Option::<Jar>::None).await;


        let players = resp.json::<Vec<Player>>().await.unwrap();

        for player in players {

            let q = "
                insert into bart.players (
                    player_name,
                    team,
                    conf,
                    gp,
                    min_per,
                    o_rtg,
                    usg,
                    e_fg,
                    ts_per,
                    orb_per,
                    drb_per,
                    ast_per,
                    to_per,
                    ftm,
                    fta,
                    ft_per,
                    two_pm,
                    two_pa,
                    two_p_per,
                    tpm,
                    tpa,
                    tp_per,
                    blk_per,
                    stl_per,
                    ftr,
                    yr,
                    ht,
                    num,
                    porpag,
                    adjoe,
                    unknown_1,
                    year,
                    pid,
                    type,
                    rec_rank,
                    ast_tov,
                    rimmade,
                    rimmade_plus_rimmiss,
                    midmade,
                    midmade_plus_midmiss,
                    rim_perc,
                    mid_perc,
                    dunksmade,
                    dunksmade_plus_dunksmiss,
                    dunks_perc,
                    pick,
                    drtg,
                    adrtg,
                    dporpag,
                    stops,
                    bpm,
                    obpm,
                    dbpm,
                    gbpm,
                    mp,
                    ogbpm,
                    dgbpm,
                    oreb,
                    dreb,
                    treb,
                    ast,
                    stl,
                    blk,
                    pts,
                    position,
                    unknown_2
                ) values (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, 
                    $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46, $47, 
                    $48, $49, $50, $51, $52, $53, $54, $55, $56, $57, $58, $59, $60, $61, $62, $63, $64, $65, $66
                ) on conflict (player_name, team, year) do update set 
                conf = excluded.conf,
                gp = excluded.gp,
                min_per = excluded.min_per,
                o_rtg = excluded.o_rtg,
                usg = excluded.usg,
                e_fg = excluded.e_fg,
                ts_per = excluded.ts_per,
                orb_per = excluded.orb_per,
                drb_per = excluded.drb_per,
                ast_per = excluded.ast_per,
                to_per = excluded.to_per,
                ftm = excluded.ftm,
                fta = excluded.fta,
                ft_per = excluded.ft_per,
                two_pm = excluded.two_pm,
                two_pa = excluded.two_pa,
                two_p_per = excluded.two_p_per,
                tpm = excluded.tpm,
                tpa = excluded.tpa,
                tp_per = excluded.tp_per,
                blk_per = excluded.blk_per,
                stl_per = excluded.stl_per,
                ftr = excluded.ftr,
                yr = excluded.yr,
                ht = excluded.ht,
                num = excluded.num,
                porpag = excluded.porpag,
                adjoe = excluded.adjoe,
                unknown_1 = excluded.unknown_1,
                year = excluded.year,
                pid = excluded.pid,
                type = excluded.type,
                rec_rank = excluded.rec_rank,
                ast_tov = excluded.ast_tov,
                rimmade = excluded.rimmade,
                rimmade_plus_rimmiss = excluded.rimmade_plus_rimmiss,
                midmade = excluded.midmade,
                midmade_plus_midmiss = excluded.midmade_plus_midmiss,
                rim_perc = excluded.rim_perc,
                mid_perc = excluded.mid_perc,
                dunksmade = excluded.dunksmade,
                dunksmade_plus_dunksmiss = excluded.dunksmade_plus_dunksmiss,
                dunks_perc = excluded.dunks_perc,
                pick = excluded.pick,
                drtg = excluded.drtg,
                adrtg = excluded.adrtg,
                dporpag = excluded.dporpag,
                stops = excluded.stops,
                bpm = excluded.bpm,
                obpm = excluded.obpm,
                dbpm = excluded.dbpm,
                gbpm = excluded.gbpm,
                mp = excluded.mp,
                ogbpm = excluded.ogbpm,
                dgbpm = excluded.dgbpm,
                oreb = excluded.oreb,
                dreb = excluded.dreb,
                treb = excluded.treb,
                ast = excluded.ast,
                stl = excluded.stl,
                blk = excluded.blk,
                pts = excluded.pts,
                position = excluded.position,
                unknown_2 = excluded.unknown_2;
                ";

            let _result = sqlx::query(q)
            .bind(player.0)
            .bind(player.1)
            .bind(player.2)
            .bind(player.3)
            .bind(player.4)
            .bind(player.5)
            .bind(player.6)
            .bind(player.7)
            .bind(player.8)
            .bind(player.9)
            .bind(player.10)
            .bind(player.11)
            .bind(player.12)
            .bind(player.13)
            .bind(player.14)
            .bind(player.15)
            .bind(player.16)
            .bind(player.17)
            .bind(player.18)
            .bind(player.19)
            .bind(player.20)
            .bind(player.21)
            .bind(player.22)
            .bind(player.23)
            .bind(player.24)
            .bind(player.25)
            .bind(player.26)
            .bind(player.27)
            .bind(player.28)
            .bind(player.29)
            .bind(player.30)
            .bind(player.31)
            .bind(player.32)
            .bind(player.33)
            .bind(player.34)
            .bind(player.35)
            .bind(player.36)
            .bind(player.37)
            .bind(player.38)
            .bind(player.39)
            .bind(player.40)
            .bind(player.41)
            .bind(player.42)
            .bind(player.43)
            .bind(player.44)
            .bind(player.45)
            .bind(player.46)
            .bind(player.47)
            .bind(player.48)
            .bind(player.49)
            .bind(player.50)
            .bind(player.51)
            .bind(player.52)
            .bind(player.53)
            .bind(player.54)
            .bind(player.55)
            .bind(player.56)
            .bind(player.57)
            .bind(player.58)
            .bind(player.59)
            .bind(player.60)
            .bind(player.61)
            .bind(player.62)
            .bind(player.63)
            .bind(player.64)
            .bind(player.65)
            .execute(&state.pool).await.unwrap();
        };
    };

    #[derive(sqlx::FromRow, Debug)]
    struct Date { day: String }

    if let Some(y) = payload.team_rankings_year {
    
        let home_url = format!("https://barttorvik.com/trank.php?year={y}#");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-agent", "Mozilla/5.0".parse().unwrap());
        let home_resp = utils::api(home_url, Method::GET, headers, vec![], {}, Option::<Jar>::None).await.text().await.unwrap();
    
        {
            let home_document = Html::parse_document(&home_resp);
    
            let tb_selector = Selector::parse("tbody").unwrap();
            let tbs = home_document.select(&tb_selector);
            let tr_selector = Selector::parse("tr").unwrap();
            let td_selector = Selector::parse("td").unwrap();
            let a_selector = Selector::parse("a").unwrap();
            // let span_selector = Selector::parse("span").unwrap();
    
            for tb in tbs {
                let trs = tb.select(&tr_selector);
    
                for tr in trs {
    
                    let tr_html = tr.inner_html();
    
                    if tr_html.contains("<th") {
                        continue;
                    }
    
                    let mut tds = tr.select(&td_selector);
                    tds.next();
                    let team = tds.next().unwrap().select(&a_selector).next().unwrap().inner_html().chars().take_while(|&ch| ch != '<').collect::<String>();
                    let conference = tds.next().unwrap().select(&a_selector).next().unwrap().inner_html();
                    tds.next();
                    let record = tds.next().unwrap().select(&a_selector).next().unwrap().inner_html();
    
                    let mut record_split = record.split("-");
                    let wins = &record_split.next().unwrap();
                    let losses = &record_split.next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let adjoe = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let adjde = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let barthag = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let efg = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let efgd = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let tor = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let tord = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let orb = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let drb = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let ftr = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let ftrd = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let _2p = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let _2pd = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let _3p = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let _3pd = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let adjt = h.split("<br>").next().unwrap();
    
                    let h: String = tds.next().unwrap().inner_html();
                    let wb = h.split("<br>").next().unwrap();
                    
                    let q = "
                    insert into bart.teams (team, conference, wins, losses, adjoe, adjde, barthag, efg, efgd, tor, tord, orb, drb,
                        ftr, ftrd, _2p, _2pd, _3p, _3pd, adjt, wb)
                    values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
                    on conflict (team) do update set 
                    conference = excluded.conference,
                    wins = excluded.wins,
                    losses = excluded.losses,
                    adjoe = excluded.adjoe,
                    adjde = excluded.adjde,
                    barthag = excluded.barthag,
                    efg = excluded.efg,
                    efgd = excluded.efgd,
                    tor = excluded.tor,
                    tord = excluded.tord,
                    orb = excluded.orb,
                    drb = excluded.drb,
                    ftr = excluded.ftr,
                    ftrd = excluded.ftrd,
                    _2p = excluded._2p,
                    _2pd = excluded._2pd,
                    _3p = excluded._3p,
                    _3pd = excluded._3pd,
                    adjt = excluded.adjt,
                    wb = excluded.wb;
                    ";
            
                    let result = sqlx::query(q)
                    .bind(team)
                    .bind(conference)
                    .bind(wins.parse::<i32>().unwrap())
                    .bind(losses.parse::<i32>().unwrap())
                    .bind(adjoe.parse::<f32>().unwrap())
                    .bind(adjde.parse::<f32>().unwrap())
                    .bind(barthag.parse::<f32>().unwrap())
                    .bind(efg.parse::<f32>().unwrap())
                    .bind(efgd.parse::<f32>().unwrap())
                    .bind(tor.parse::<f32>().unwrap())
                    .bind(tord.parse::<f32>().unwrap())
                    .bind(orb.parse::<f32>().unwrap())
                    .bind(drb.parse::<f32>().unwrap())
                    .bind(ftr.parse::<f32>().unwrap())
                    .bind(ftrd.parse::<f32>().unwrap())
                    .bind(_2p.parse::<f32>().unwrap())
                    .bind(_2pd.parse::<f32>().unwrap())
                    .bind(_3p.parse::<f32>().unwrap())
                    .bind(_3pd.parse::<f32>().unwrap())
                    .bind(adjt.parse::<f32>().unwrap())
                    .bind(wb.parse::<f32>().unwrap())
                    .execute(&state.pool);
    
                    v.push(result);
    
                }
            }
    
        }
    }

    if let (Some(start), Some(end)) = (payload.game_start_date, payload.game_end_date) {
        let q = &format!("
        SELECT replace(date_trunc('day', dd)::date::varchar, '-', '') as day
        FROM generate_series
                ( '{start}'::timestamp
                , '{end}'::timestamp
                , '1 day'::interval) dd
                ;
        ");
    
        let result = sqlx::query_as::<_, Date>(q).fetch_all(&state.pool).await.unwrap();
        let mut days = Vec::new();
    
        let mut responses = Vec::new();
        for day in result {
            days.push(day.day.clone());
            let today = day.day;
            let url = format!("https://barttorvik.com/schedule.php?date={today}").parse::<Url>().unwrap();
    
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("User-agent", "Mozilla/5.0".parse().unwrap());
            let resp = utils::api(url, Method::GET, headers, vec![], {}, Option::<Jar>::None).await.text().await.unwrap();
            responses.push(resp)
        }
    
        'days: for (idx,response) in responses.iter().enumerate() {
        
    
            let doc = Html::parse_document(&response);
            
            let tb_selector = Selector::parse("#tblData > tbody").unwrap();
            let tb = doc.select(&tb_selector).next().unwrap();
        
            let tr_selector = Selector::parse("tr").unwrap();
            let trs = tb.select(&tr_selector);
            let td_selector = Selector::parse("td").unwrap();
            let a_selector = Selector::parse("a").unwrap();
            let span_selector = Selector::parse("span").unwrap();
    
            for tr in trs {
    
                let tr_html = tr.inner_html();
                if tr_html.contains("MOV Mean absolute error") {
                    continue 'days;
                }
    
                let mut tds = tr.select(&td_selector);
                tds.next();
                let matchup_td = tds.next().unwrap();
                let mut a_s = matchup_td.select(&a_selector);
                let mut spans = matchup_td.select(&span_selector);
                let away_team_rank = spans.next().unwrap().inner_html().parse::<i32>().unwrap_or(0);
                let away_team = a_s.next().unwrap().inner_html();
                spans.next();
                // let neutral_site_indicator = 
                let home_team_rank = spans.next().unwrap().inner_html().parse::<i32>().unwrap_or(0);
                let home_team = a_s.next().unwrap().inner_html();
    
                let matchup_td_text = matchup_td.inner_html();
                let neutral_site = matchup_td_text.contains("vs.");
    
                let trank_a = tds.next().unwrap().select(&a_selector).next().unwrap().inner_html().replace("<span class=\"mobileonly\">,<br></span>", "");
    
                let (home_team_projection, away_team_projection, home_team_projected_score, away_team_projected_score, spread) = if home_team_rank != 0 && away_team_rank != 0 {
                    let mut a_split = trank_a.rsplit(" ");
                    let winning_team_projection = a_split.next().unwrap()[1..3].parse::<f64>().unwrap_or(0.0);
                    let mut scores_string_split = a_split.next().unwrap().split("-");
                    let winning_score = scores_string_split.next().unwrap().parse::<i32>().unwrap_or(0);
                    let losing_score = scores_string_split.next().unwrap().parse::<i32>().unwrap_or(0);
                    let spread = a_split.next().unwrap().parse::<f64>().unwrap_or(0.0);
    
                    let mut projected_winner_words: Vec<&str> = a_split.collect();
                    projected_winner_words.reverse();
                    let winning_team = projected_winner_words.join(" ");
    
                    let home_team_projection = if winning_team == home_team {winning_team_projection} else {100.0 - winning_team_projection};
                    let away_team_projection = if winning_team == away_team {winning_team_projection} else {100.0 - winning_team_projection};
        
                    let home_team_projected_score = if winning_team == home_team {winning_score} else {losing_score};
                    let away_team_projected_score = if winning_team == away_team {winning_score} else {losing_score};
        
                    (home_team_projection, away_team_projection, home_team_projected_score, away_team_projected_score, spread)
                } else {
                    (0.0, 0.0, 0, 0, 0.0)
                };
    
                let thrill_score = tds.next().unwrap().inner_html().parse::<f64>().unwrap_or(0.0);
    
                let q = "
                insert into bart.games (game_date, home_team, away_team, neutral_site, home_team_projected_score, 
                    home_team_projection, home_team_rank, away_team_projected_score, away_team_projection, away_team_rank, thrill_score, spread)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                on conflict (game_date, home_team, away_team) do update set 
                neutral_site = excluded.neutral_site,
                home_team_projected_score = excluded.home_team_projected_score,
                home_team_projection = excluded.home_team_projection,
                away_team_projected_score = excluded.away_team_projected_score,
                away_team_projection = excluded.away_team_projection,
                thrill_score = excluded.thrill_score,
                home_team_rank = excluded.home_team_rank,
                away_team_rank = excluded.away_team_rank,
                spread = excluded.spread;
                ";
        
                let result = sqlx::query(q)
                .bind(days[idx].clone())
                .bind(home_team)
                .bind(away_team)
                .bind(neutral_site)
                .bind(home_team_projected_score)
                .bind(home_team_projection)
                .bind(home_team_rank)
                .bind(away_team_projected_score)
                .bind(away_team_projection)
                .bind(away_team_rank)
                .bind(thrill_score)
                .bind(spread)
                .execute(&state.pool);
    
                v.push(result);
            }
        }

    };


    let stream = futures::stream::iter(v).buffer_unordered(10);
    let results = stream.collect::<Vec<_>>().await;

    for r in results {
        if r.is_err() {
            println!("{:?}", r);
        }
    }



    "Success"
}