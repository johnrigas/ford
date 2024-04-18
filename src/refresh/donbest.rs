use axum::extract::State;
use axum_macros::debug_handler;
use reqwest::Method;
use serde::Deserialize;
use crate::utils as main_utils;
use crate::refresh::utils;
use reqwest::cookie::Jar;


#[derive(Deserialize, Debug)]
struct Location {

}

#[derive(Deserialize, Debug)]
struct Injury {
    playerId: String,
    playerFirstName: String,
    playerLastName: String,
    teamId: String,
    injury: String,
    startDate: String,
    status: String,
    displayStatus: String,
    note: String,
    lastUpdated: String,
    team: Vec<Team>,
}

#[derive(Deserialize, Debug)]
struct Team {
    teamId: String,
    teamName: String,
    teamShortName: String,
    location: Location,
    r#type: String
}

#[derive(Deserialize, Debug)]
struct TeamInjury {
    teamId: String,
    injuries: Vec<Injury>,
    team: Vec<Team>
}

#[derive(Deserialize, Debug)]
struct Injuries {
    teamInjuries: Vec<TeamInjury>
}

#[derive(Deserialize, Debug)]
struct InjuryResponse {
    injuries: Injuries
}


#[debug_handler]
pub async fn refresh_donbest(State(state): State<main_utils::AppState>) -> &'static str {

    let home_url = "https://prod-ghosts-api-widgets.prod.sports.gracenote.com/api/Injuries?customerId=1412&editionId=%2Fsport%2Fbasketball%2Fseason:664652&filter=%7B%22include%22:%5B%22team%22,%22players%22%5D,%22fields%22:%7B%22teamInjuries%22:%7B%22injuries%22:%7B%22playerId%22:true,%22location%22:true,%22teamId%22:true,%22startDate%22:true,%22injury%22:true,%22status%22:true,%22displayStatus%22:true,%22note%22:true,%22lastUpdated%22:true%7D%7D,%22players%22:%7B%22playerFirstName%22:true,%22playerLastName%22:true,%22thumbnailUrl%22:true,%22seasonDetails%22:%7B%22position%22:%7B%22positionShortName%22:true%7D%7D%7D,%22team%22:%7B%22teamName%22:true,%22teamShortName%22:true,%22type%22:true%7D%7D%7D&languageCode=2&module=na_teamsports&sportId=%2Fsport%2Fbasketball&type=injuries";
    let params = vec![];

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("Referer", "https://widgets.sports.gracenote.com/".parse().unwrap());
    let resp = utils::api(home_url, Method::GET, headers, params, {}, Option::<Jar>::None).await;

    let injury_response = resp.json::<InjuryResponse>().await.unwrap();

    for team in injury_response.injuries.teamInjuries {
        for i in team.injuries {

            let team_id = i.teamId;
            let player_id = i.playerId;
            let team_name = &i.team[0].teamName;
            let player_first_name = i.playerFirstName;
            let player_last_name = i.playerLastName;
            let injury = i.injury;
            let start_date = i.startDate;
            let status = i.status;
            let display_status = i.displayStatus;
            let note = i.note;
            let last_updated = i.lastUpdated;

            let mut empty_q =  String::new();
            let q = main_utils::upsert! (
                empty_q;
                &state.pool;
                "donbest.injuries"
                team_id, player_id, team_name, player_first_name, player_last_name, injury, 
                start_date, status, display_status, note, last_updated; 
                conflict("team_id", "player_id", "player_first_name", "player_last_name", "team_name", "injury")
            );
        
            let _result = q.await.unwrap();

        }
    }

    "Success"
}