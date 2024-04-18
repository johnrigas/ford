use axum::extract::State;
use axum_macros::debug_handler;
use futures::TryFutureExt;
use reqwest::Method;
use serde::Deserialize;
use crate::utils::AppState;
use crate::refresh::utils;
use std::collections::HashMap;
use reqwest::cookie::Jar;
use std::fs;


#[derive(Deserialize, Debug)]
struct UserLegacy {
    name: String,
    screen_name: String
}

#[derive(Deserialize, Debug)]
struct Result3 {
    id: String,
    legacy: UserLegacy
}


#[derive(Deserialize, Debug)]
struct UserResults {
    result: Result3
}


#[derive(Deserialize, Debug)]
struct Core {
    user_results: UserResults
}


#[derive(Deserialize, Debug)]
struct Legacy {
    full_text: String,
    created_at: String
}

#[derive(Deserialize, Debug)]
struct TweetResult {
    core: Core,
    legacy: Legacy

}

// #[derive(Deserialize, Debug)]
// struct Result2 {
//     core: Core,
//     legacy: Legacy

// }


// #[derive(Deserialize, Debug)]
// struct QuotedStatusResult {
//     result: Result2

// }

// #[derive(Deserialize, Debug)]
// struct TweetResult {
//     quoted_status_result: Option<QuotedStatusResult>

// }


#[derive(Deserialize, Debug)]
struct TweetResults {
    result: TweetResult

}


#[derive(Deserialize, Debug)]
struct ItemContent {
    tweet_results: TweetResults,
    itemType: String

}

#[derive(Deserialize, Debug)]
struct EntryContent {
    itemContent: Option<ItemContent>,
    entryType: String

}


#[derive(Deserialize, Debug)]
struct Entry {
    entryId: String,
    content: EntryContent
}



#[derive(Deserialize, Debug)]
struct Instruction {
    r#type: String,
    entry: Option<Entry>,
    entries: Option<Vec<Entry>>
}

#[derive(Deserialize, Debug)]
struct Timeline {
    instructions: Vec<Instruction>
}


#[derive(Deserialize, Debug)]
struct TimelineV2 {
    timeline: Timeline
}

#[derive(Deserialize, Debug)]
struct Result {
    timeline_v2: TimelineV2
}

#[derive(Deserialize, Debug)]
struct User {
    result: Result
}


#[derive(Deserialize, Debug)]
struct UserTweetsData {
    user: User
}

#[derive(Deserialize, Debug)]
struct UserTweetsResponse {
    data: UserTweetsData
}


#[debug_handler]
pub async fn refresh_twitter(State(state): State<AppState>) -> &'static str {
    // let c = 40;

    let url = format!("https://twitter.com/i/api/graphql/V1ze5q3ijDS1VeLwLY0m7g/UserTweets?variables=%7B%22userId%22%3A%2285605195%22%2C%22count%22%3A20%2C%22includePromotedContent%22%3Atrue%2C%22withQuickPromoteEligibilityTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%2C%22withV2Timeline%22%3Atrue%7D&features=%7B%22responsive_web_graphql_exclude_directive_enabled%22%3Atrue%2C%22verified_phone_label_enabled%22%3Afalse%2C%22creator_subscriptions_tweet_preview_api_enabled%22%3Atrue%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%2C%22responsive_web_graphql_skip_user_profile_image_extensions_enabled%22%3Afalse%2C%22c9s_tweet_anatomy_moderator_badge_enabled%22%3Atrue%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22responsive_web_edit_tweet_api_enabled%22%3Atrue%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Atrue%2C%22view_counts_everywhere_api_enabled%22%3Atrue%2C%22longform_notetweets_consumption_enabled%22%3Atrue%2C%22responsive_web_twitter_article_tweet_consumption_enabled%22%3Afalse%2C%22tweet_awards_web_tipping_enabled%22%3Afalse%2C%22freedom_of_speech_not_reach_fetch_enabled%22%3Atrue%2C%22standardized_nudges_misinfo%22%3Atrue%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Atrue%2C%22rweb_video_timestamps_enabled%22%3Atrue%2C%22longform_notetweets_rich_text_read_enabled%22%3Atrue%2C%22longform_notetweets_inline_media_enabled%22%3Atrue%2C%22responsive_web_media_download_video_enabled%22%3Afalse%2C%22responsive_web_enhance_cards_enabled%22%3Afalse%7D");
    let params = vec![];

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("authority", "twitter.com".parse().unwrap());
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA".parse().unwrap());
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("cookie", "guest_id=v1%3A169989511216369517; guest_id_marketing=v1%3A169989511216369517; guest_id_ads=v1%3A169989511216369517; gt=1740441563258298664; _ga=GA1.2.1653262638.1703788581; _gid=GA1.2.2106239854.1703788581; g_state={\"i_l\":0}; kdt=SKg4nQF4R21CQEc2xIMXee6xy0wAayuV1AFjiaiO; auth_token=6571b4aabe6e88a62a114ea9cf4955666f0443c7; ct0=1d7641343da2c3e861075771727b45d5d944d959718b3a7be9a820e8469149329bbfabab2e885088fa5b52db4c162362c5cddffc637c6c01d1501cd654c4b9d95ed2dbfffa45acca22fa0ec3825aa23c; lang=en; twid=u%3D1499377291; att=1-1ZrHHFwvXRWzbZdEnOBnZN9FVe983ZYhdUiETEmb; personalization_id=\"v1_ilwnbFy/bu3iLuLPacQYxA==\"".parse().unwrap());
    headers.insert("referer", "https://twitter.com/JonRothstein".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("x-client-transaction-id", "A6StN9xu6GxGhWr7QTMNBcBe/zhRvLmUWuLJlVFMQfk0uKyX6qHpQJAY48IM6K9Sk9xiPQI1ShZ+3h9cGhygozopD84aAg".parse().unwrap());
    headers.insert("x-client-uuid", "1033df48-f631-48e5-a3d1-287318f3ad98".parse().unwrap());
    headers.insert("x-csrf-token", "1d7641343da2c3e861075771727b45d5d944d959718b3a7be9a820e8469149329bbfabab2e885088fa5b52db4c162362c5cddffc637c6c01d1501cd654c4b9d95ed2dbfffa45acca22fa0ec3825aa23c".parse().unwrap());
    headers.insert("x-twitter-active-user", "yes".parse().unwrap());
    headers.insert("x-twitter-auth-type", "OAuth2Session".parse().unwrap());
    headers.insert("x-twitter-client-language", "en".parse().unwrap());

    let resp = utils::api(url, Method::GET, headers, params, {}, Option::<Jar>::None).await;

    // println!("{:?}", resp.text().await.unwrap());
    let user_tweets_resp = resp.json::<UserTweetsResponse>().await.unwrap();

    let instructions = user_tweets_resp.data.user.result.timeline_v2.timeline.instructions;

    for instruction in instructions {

        if instruction.r#type == "TimelineAddEntries" {
            let entries = instruction.entries.unwrap();
            for entry in entries {

                if entry.content.entryType == "TimelineTimelineItem" {

                    let item_content = entry.content.itemContent.unwrap();

                    if item_content.itemType == "TimelineTweet" {

                        let result = item_content.tweet_results;

                        // println!("{:?}", &entry.entryId);
                        // println!("{:?}", &result.result.legacy.full_text);
                        // println!("{:?}", &result.result.core.user_results.result.id);
                        // println!("{:?}", &result.result.core.user_results.result.legacy.name);
                        // println!("{:?}", &result.result.core.user_results.result.legacy.screen_name);

                        let q = "
                        insert into twitter.tweets (user_id, user_name, user_screen_name, full_text, tweet_id, tweet_time)
                        values ($1, $2, $3, $4, $5, $6)
                        on conflict (user_id, tweet_id) do update set 
                        user_name = excluded.user_name,
                        user_screen_name = excluded.user_screen_name,
                        full_text = excluded.full_text,
                        tweet_time = excluded.tweet_time;
                        ";
                
                        let _result = sqlx::query(q)
                        .bind(result.result.core.user_results.result.id)
                        .bind(result.result.core.user_results.result.legacy.name)
                        .bind(result.result.core.user_results.result.legacy.screen_name)
                        .bind(result.result.legacy.full_text)
                        .bind(entry.entryId)
                        .bind(result.result.legacy.created_at)
                        .execute(&state.pool).await.unwrap();
            
                    }
                    
                }

            }

        }
    }



    "Success"
}


