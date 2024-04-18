mod refresh;
mod utils;

use axum::{
    routing::{get,post}, Router, extract::State, Json
};
use refresh::summary::refresh_summary;
use refresh::bart::{refresh_bart, BartRefreshPayload};
use refresh::kenpom::{refresh_kenpom, KenpomRefreshPayload};
use refresh::espn::{refresh_espn, ESPNRefreshPayload};
use refresh::fanduel::refresh_fanduel;
use refresh::donbest::refresh_donbest;
use refresh::twitter::refresh_twitter;
use refresh::vegas::refresh_vegas;
// use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use chrono::{DateTime, Utc};
use tokio_cron_scheduler::{JobScheduler, Job};
use chrono;

#[tokio::main]
async fn main() {

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();


    let mut sched = JobScheduler::new().await.unwrap();

    sched.add(Job::new_async("0 59 * * * *", |_uuid, mut _l| Box::pin(async move {
        let today = chrono::offset::Local::now().date_naive().to_string();
        println!("Running espn {:?} - {:?}", today, today);
        let pool2 = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();

        let app_state = utils::AppState { pool: pool2.clone() };

        let payload = ESPNRefreshPayload {
            game_start_date: Some(today.clone()),
            game_end_date: Some(today)
        };
        refresh_espn(State(app_state), Json(payload)).await;
       
        pool2.close().await;
        println!("Done with espn");

    })).unwrap()).await.unwrap();
  
    sched.add(Job::new_async("0 0 * * * *", |_uuid, mut _l| Box::pin(async move {
        println!("Running twitter");
        let pool2 = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();

        let app_state = utils::AppState { pool: pool2.clone() };
        refresh_twitter(State(app_state)).await;
       
        pool2.close().await;
        println!("Done with twitter");

    })).unwrap()).await.unwrap();

    sched.add(Job::new_async("0 1 * * * *", |_uuid, mut _l| Box::pin(async move {
        println!("Running fanduel");
        let pool2 = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();

        let app_state = utils::AppState { pool: pool2.clone() };
        refresh_fanduel(State(app_state)).await;
       
        pool2.close().await;
        println!("Done with fanduel");

    })).unwrap()).await.unwrap();


    sched.add(Job::new_async("0 2 * * * *", |_uuid, mut _l| Box::pin(async move {
        println!("Running donbest");
        let pool2 = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();

        let app_state = utils::AppState { pool: pool2.clone() };
        refresh_donbest(State(app_state)).await;
       
        pool2.close().await;
        println!("Done with donbest");

    })).unwrap()).await.unwrap();


    sched.add(Job::new_async("0 3 * * * *", |_uuid, mut _l| Box::pin(async move {
        let today = chrono::offset::Local::now().date_naive().to_string();
        println!("Running bart {:?} - {:?}", today, today);
        let pool2 = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();

        let app_state = utils::AppState { pool: pool2.clone() };

        let payload = BartRefreshPayload {
            player_stats_year: Option::<String>::None,
            team_rankings_year: Some("2024".to_string()),
            game_start_date: Some(today.clone()),
            game_end_date: Some(today)
        };
        refresh_bart(State(app_state), Json(payload)).await;
       
        pool2.close().await;
        println!("Done with bart");

    })).unwrap()).await.unwrap();


    // sched.add(Job::new_async("0 4 * * * *", |_uuid, mut _l| Box::pin(async move {
    //     let today = chrono::offset::Local::now().date_naive().to_string();
    //     println!("Running Kenpom {:?} - {:?}", today, today);
    //     let pool2 = PgPoolOptions::new()
    //     .max_connections(1)
    //     .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();

    //     let app_state = utils::AppState { pool: pool2.clone() };

    //     let payload = KenpomRefreshPayload {
    //         team_rankings_year: Some("2024".to_string()),
    //         game_start_date: Some(today.clone()),
    //         game_end_date: Some(today),
    //         session_id_cookie: "5d5409604cb0fead8791750b3201f948".to_string()
    //     };
    //     refresh_kenpom(State(app_state), Json(payload)).await;
       
    //     pool2.close().await;
    //     println!("Done with Kenpom");

    // })).unwrap()).await.unwrap();


    sched.add(Job::new_async("0 5 * * * *", |_uuid, mut _l| Box::pin(async move {
        println!("Running Vegas");
        let pool2 = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();

        let app_state = utils::AppState { pool: pool2.clone() };
        refresh_vegas(State(app_state)).await;
       
        pool2.close().await;
        println!("Done with vegas");

    })).unwrap()).await.unwrap();

    // sched.add(Job::new_async("0 7 * * * *", |_uuid, mut _l| Box::pin(async move {
    //     println!("Running Summary");
    //     let pool2 = PgPoolOptions::new()
    //     .max_connections(1)
    //     .connect("postgres://postgres:postgres@localhost:5330/ford").await.unwrap();

    //     let app_state = utils::AppState { pool: pool2.clone() };
    //     refresh_summary(State(app_state)).await;
       
    //     pool2.close().await;
    //     println!("Done with summary");

    // })).unwrap()).await.unwrap();



    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    sched.set_shutdown_handler(Box::new(|| {
      Box::pin(async move {
        println!("Shut down done");
      })
    }));

    sched.start().await.unwrap();


    tracing_subscriber::fmt::init();


    let app = Router::new()
        .route("/", get(root))
        .route("/refresh/espn", post(refresh_espn))
        .route("/refresh/fanduel", post(refresh_fanduel))
        .route("/refresh/summary", post(refresh_summary))
        .route("/refresh/kenpom", post(refresh_kenpom))
        .route("/refresh/bart", post(refresh_bart))
        .route("/refresh/donbest", post(refresh_donbest))
        .route("/refresh/twitter", post(refresh_twitter))
        .route("/refresh/vegas", post(refresh_vegas))
        .with_state(utils::AppState { pool: (pool) });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();



}
// basic handler that responds with a static string
async fn root(State(state): State<utils::AppState>) -> &'static str {
    let row: (DateTime<Utc>,) = sqlx::query_as("SELECT current_timestamp")
        .fetch_one(&state.pool).await.unwrap();

    println!("{:?}", row);

    "Just johnnyo amato"
}