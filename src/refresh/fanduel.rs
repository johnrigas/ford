use axum::extract::State;
use axum_macros::debug_handler;
use reqwest::Method;
use serde::Deserialize;
use crate::utils::AppState;
use crate::refresh::utils;
use std::collections::HashMap;
use reqwest::cookie::Jar;
use std::fs;

#[derive(Deserialize, Debug)]
struct Event {
    eventId: i64,
    marketIds: Vec<Option<String>>
}

#[derive(Deserialize, Debug)]
struct Display {
    marketLayoutTemplate: String,
    competitionId: i64,
    rows: Vec<Event>
}

#[derive(Deserialize, Debug)]
struct Coupon {
    display: Option<Vec<Display>>,
    attachmentsDependent: Option<bool>,
    attachmentsFullyLoaded: Option<bool>,
    id: Option<i64>,
    eventTypeId: Option<i64>,    
    filter: Option<Vec<String>>,
    marketTypes: Option<Vec<String>>,
    hasMarketSwitcher: Option<bool>,
    title: Option<String>,
    competitionIds: Option<Vec<i64>>,
    hasAttachments: Option<bool>
}

#[derive(Deserialize, Debug)]
struct FractionalOdds {
    numerator: i64,
    denominator: i64
}

#[derive(Deserialize, Debug)]
struct DecimalOdds {
    decimalOdds: f64
}

#[derive(Deserialize, Debug)]
struct AmericanDisplayOdds {
    americanOdds: i64,
    americanOddsInt: i64
}


#[derive(Deserialize, Debug)]
struct TrueOdds {
    decimalOdds: DecimalOdds,
    fractionalOdds: FractionalOdds
}

#[derive(Deserialize, Debug)]
struct WinRunnerOdds {
    americanDisplayOdds: AmericanDisplayOdds,
    trueOdds: TrueOdds
}

#[derive(Deserialize, Debug)]
struct Result {
    r#type: Option<String>
}

#[derive(Deserialize, Debug)]
struct Runner {
    selectionId: i64,
    handicap: f64,
    runnerName: String,
    sortPriority: i64,
    result: Result,
    runnerStatus: String,
    nameAbbr: Option<String>,
    winRunnerOdds: WinRunnerOdds
    // previousWinRunnerOdds: Vec<_>
}


#[derive(Deserialize, Debug)]
struct Market {
    marketId: String,
    eventTypeId: i64,
    competitionId: i64,
    eventId: i64,    
    marketName: String,
    marketTime: String,
    marketType: String,
    bspMarket: bool,
    sgmMarket: bool,
    inPlay: bool,
    numberOfRunners: i64,
    numberOfActiveRunners: i64,
    numberOfWinners: i64,
    sortPriority: i64,
    bettingType: String,
    marketStatus: String,
    marketLevels: Vec<String>,
    runners: Vec<Runner>,
    canTurnInPlay: bool,
    associatedMarkets: Vec<AssociatedMarket>,
    shouldDisplayStatsInRunner: bool,
    eachwayAvailable: bool,
    legTypes: Vec<String>
}

#[derive(Deserialize, Debug)]
struct AssociatedMarket {
    eventId: i64,
    eventTypeId: i64,
    externalMarketId: String
}

#[derive(Deserialize, Debug)]
struct Layout {
    coupons: HashMap<String, Coupon>
}

#[derive(Deserialize, Debug)]
struct Attachment {
    markets: HashMap<String, Market>
}


#[derive(Deserialize, Debug)]
struct ContentManagedPage {
    layout: Layout,
    attachments: Attachment
}


#[debug_handler]
pub async fn refresh_fanduel(State(state): State<AppState>) -> &'static str {
    let url = "https://sbapi.co.sportsbook.fanduel.com/api/content-managed-page";
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("authority", "sbapi.co.sportsbook.fanduel.com".parse().unwrap());
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("origin", "https://sportsbook.fanduel.com".parse().unwrap());
    headers.insert("referer", "https://sportsbook.fanduel.com/".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Google Chrome\";v=\"119\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36".parse().unwrap());

    let response = utils::api(url, Method::GET, headers, vec![
        ("page", "CUSTOM"),
        ("customPageId", "ncaab"),
        ("pbHorizontal", "false"),
        ("_ak", "FhMFpcPWXMeyZxOx"),
        ("timezone", "America%2FDenver")
    ], {}, Option::<Jar>::None).await;

    // let response_body = response.text().await.unwrap();
    // fs::write("dump.json", response_body).expect("Unable to write file");
    // panic!();
    let content_managed_page = response.json::<ContentManagedPage>().await.unwrap();

    let coupon = content_managed_page.layout.coupons.values().find(|c| c.title.as_ref().unwrap_or(&"".to_string()) == "NCAAB All").unwrap();
    // let display = coupon.display.as_ref().unwrap().first().unwrap();
    // let events: &Vec<Event> = display.rows.as_ref();
    // let market_ids: Vec<&String> = events.iter().flat_map(|r| -> &Vec<String> {r.marketIds.as_ref()}).collect();
    let mut market_ids = Vec::new();

    for display in coupon.display.as_ref().unwrap() {
        let events: &Vec<Event> = display.rows.as_ref();
        let mut new_market_ids: Vec<&Option<String>> = events.iter().flat_map(|r| -> &Vec<Option<String>> {r.marketIds.as_ref()}).collect();
        market_ids.append(&mut new_market_ids);
    }

    let not_null_market_ids: Vec<&&Option<String>> = market_ids.iter().filter(|m| m.is_some()).collect();
    let not_null_market_ids: Vec<&String> = not_null_market_ids.iter().map(|m| m.as_ref().clone().unwrap()).collect();

    // let real_market_ids: &Vec<Event> = coupon.display.as_ref().unwrap().iter().flat_map(|d| d.rows.as_ref()).collect();

    // let url = "https://smp.co.sportsbook.fanduel.com/api/sports/fixedodds/readonly/v1/getMarketPrices?priceHistory=1";
    // let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert("Content-Type", "application/json".parse().unwrap());
    // let mut j = HashMap::<&str, Vec<&String>>::new();
    // j.insert("marketIds", market_ids);
    // let response = utils::api(url, Method::POST, headers, vec![], j).await;
    // let b: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    let markets = content_managed_page.attachments.markets;
    for (market_id, market) in markets {
        if not_null_market_ids.contains(&&market_id) {

            let q = "
            insert into fanduel.markets (market_id, event_id, market_type, market_status, market_time, betting_type, market_name)
            values ($1, $2, $3, $4, $5, $6, $7)
            on conflict (market_id) do update set 
            event_id = excluded.event_id,
            market_type = excluded.market_type,
            market_status = excluded.market_status,
            market_time = excluded.market_time,
            betting_type = excluded.betting_type,
            market_name = excluded.market_name;
            ";
    
            let _result = sqlx::query(q)
            .bind(&market_id)
            .bind(market.eventId)
            .bind(market.marketType)
            .bind(market.marketStatus)
            .bind(market.marketTime)
            .bind(market.bettingType)
            .bind(market.marketName)
            .execute(&state.pool).await.unwrap();
    
            for runner in market.runners {

                let q = "
                insert into fanduel.runners (market_id, selection_id, handicap, runner_name, runner_abbreviation, result_type, runner_status, american_odds, american_odds_int)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                on conflict (market_id, selection_id) do update set 
                handicap = excluded.handicap,
                runner_name = excluded.runner_name,
                runner_abbreviation = excluded.runner_abbreviation,
                result_type = excluded.result_type,
                runner_status = excluded.runner_status,
                american_odds = excluded.american_odds,
                american_odds_int = excluded.american_odds_int;
                ";
        
                let _result = sqlx::query(q)
                .bind(&market_id)
                .bind(runner.selectionId)
                .bind(runner.handicap)
                .bind(runner.runnerName)
                .bind(runner.nameAbbr)
                .bind(runner.result.r#type)
                .bind(runner.runnerStatus)
                .bind(runner.winRunnerOdds.americanDisplayOdds.americanOdds)
                .bind(runner.winRunnerOdds.americanDisplayOdds.americanOddsInt)
                .execute(&state.pool).await.unwrap();

            }
            
            
        }

    }




    "Success"
}
 

