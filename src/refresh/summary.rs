use axum::extract::State;
use axum_macros::debug_handler;
use crate::utils::AppState;


#[debug_handler]
pub async fn refresh_summary(State(state): State<AppState>) -> &'static str {

    let q = "
    insert into summary.names (clean_name, name_type)
    select distinct clean_team_name(location), 'team'
    from espn.teams
    on conflict (clean_name, name_type) do nothing;
    ";

    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();
    let q = "
    insert into summary.name_mappings (clean_name, name_type, source, source_name, source_id)
    select n.clean_name, 'team', 'espn', t.location, t.team_id
    from summary.names n
    join espn.teams t on clean_team_name(t.location) = n.clean_name
    on conflict (clean_name, name_type, source) do nothing;
    ";

    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();


    let q = "
    insert into summary.name_mappings (clean_name, name_type, source, source_name, source_id, source_abbreviation)
    select distinct n.clean_name, 'team', 'fanduel', r.runner_name, null, r.runner_abbreviation
    from summary.names n
    join fanduel.runners r on clean_team_name(r.runner_name) = n.clean_name
    on conflict (clean_name, name_type, source) do nothing;
    ";

    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();

    let q = "
    insert into summary.name_mappings (clean_name, name_type, source, source_name, source_id)
    select distinct n.clean_name, 'team', 'kenpom', t.team, null
    from summary.names n
    join kenpom.teams t on clean_team_name(
        replace(case
            when t.team = 'Mississippi' then 'olemiss'
            when t.team = 'Saint Francis' then 'stfrancispa'
            when t.team = 'Cal Baptist' then 'californiabaptist'
            when t.team = 'Loyola MD' then 'loyolamaryland'
            when t.team = 'American' then 'americanuniversity'
            when t.team = 'Queens' then 'queensuniversity'
            when t.team = 'Miami FL' then 'miami'
            when t.team = 'Tennessee Martin' then 'utmartin'
            when t.team = 'Sam Houston St.' then 'samhouston'
            when t.team = 'USC Upstate' then 'southcarolinaupstate'
            when t.team = 'Texas A&amp;M Corpus Chris' then 'texasamcorpuschristi'
            when t.team = 'Illinois Chicago' then 'UIC'
            when t.team = 'San Jose St.' then 'sanjosstate'
            when t.team = 'North Carolina A&amp;T' then 'North Carolina A&T'
            when t.team = 'Nicholls St.' then 'Nicholls'
            when t.team = 'Southeastern Louisiana' then 'SE Louisiana'
            when t.team = 'McNeese St.' then 'McNeese'
            when t.team = 'William &amp; Mary' then 'William & Mary'
            when t.team = 'Texas A&amp;M Commerce' then 'Texas A&M Commerce'
            when t.team = 'Albany' then 'U Albany'
            when t.team = 'Seattle' then 'Seattle U'
            when t.team = 'FIU' then 'floridainternational'
            when t.team = 'Connecticut' then 'uconn'
            when t.team = 'Grambling St.' then 'Grambling'
            when t.team = 'Penn' then 'pennsylvania'
            when t.team = 'LIU' then 'longislanduniversity'
            when t.team = 'Nebraska Omaha' then 'Omaha'
            when t.team = 'St. Thomas' then 'St. Thomas Minnesota'
            when t.team = 'UMKC' then 'Kansas City'
            when t.team = 'Louisiana Monroe' then 'UL Monroe'
            when RIGHT(t.team, 3) = 'St.' then replace(t.team, 'St.', 'State')
            when RIGHT(t.team, 7) = 'A&amp;M' then replace(t.team, 'A&amp;M', 'am')
            else t.team end, 'Cal St.', 'Cal State')
        ) = n.clean_name
    on conflict (clean_name, name_type, source) do nothing;
    ";
    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();


    let q = "
    insert into summary.name_mappings (clean_name, name_type, source, source_name, source_id)
    select distinct n.clean_name, 'team', 'bart', t.team, null
    from summary.names n
    join bart.teams t on clean_team_name(
        replace(case
            when t.team = 'Mississippi' then 'olemiss'
            when t.team = 'Saint Francis' then 'stfrancispa'
            when t.team = 'Cal Baptist' then 'californiabaptist'
            when t.team = 'Loyola MD' then 'loyolamaryland'
            when t.team = 'American' then 'americanuniversity'
            when t.team = 'Queens' then 'queensuniversity'
            when t.team = 'Miami FL' then 'miami'
            when t.team = 'Tennessee Martin' then 'utmartin'
            when t.team = 'Sam Houston St.' then 'samhouston'
            when t.team = 'USC Upstate' then 'southcarolinaupstate'
            when t.team = 'Texas A&amp;M Corpus Chris' then 'texasamcorpuschristi'
            when t.team = 'Illinois Chicago' then 'UIC'
            when t.team = 'San Jose St.' then 'sanjosstate'
            when t.team = 'North Carolina A&amp;T' then 'North Carolina A&T'
            when t.team = 'Nicholls St.' then 'Nicholls'
            when t.team = 'Southeastern Louisiana' then 'SE Louisiana'
            when t.team = 'McNeese St.' then 'McNeese'
            when t.team = 'William &amp; Mary' then 'William & Mary'
            when t.team = 'Texas A&amp;M Commerce' then 'Texas A&M Commerce'
            when t.team = 'Albany' then 'U Albany'
            when t.team = 'Seattle' then 'Seattle U'
            when t.team = 'FIU' then 'floridainternational'
            when t.team = 'Connecticut' then 'uconn'
            when t.team = 'Grambling St.' then 'Grambling'
            when t.team = 'Penn' then 'pennsylvania'
            when t.team = 'LIU' then 'longislanduniversity'
            when t.team = 'Nebraska Omaha' then 'Omaha'
            when t.team = 'St. Thomas' then 'St. Thomas Minnesota'
            when t.team = 'UMKC' then 'Kansas City'
            when t.team = 'Louisiana Monroe' then 'UL Monroe'
            when t.team = 'College of Charleston' then 'Charleston'
            when t.team = 'North Carolina St.' then 'NC State'
            when t.team = 'Detroit' then 'detroitmercy'
            when t.team = 'Fort Wayne' then 'purduefortwayne'
            when t.team = 'LIU Brooklyn' then 'longislanduniversity'
            when t.team = 'Louisiana Lafayette' then 'Louisiana'
            when RIGHT(t.team, 3) = 'St.' then replace(t.team, 'St.', 'State')
            when RIGHT(t.team, 7) = 'A&amp;M' then replace(t.team, 'A&amp;M', 'am')
            else t.team end, 'Cal St.', 'Cal State')
        ) = n.clean_name
    on conflict (clean_name, name_type, source) do nothing;
    ";

    let q = "
    insert into summary.games (game, source, over_under, over_odds, under_odds, home_moneyline, away_moneyline,
        spread, home_spread_odds, away_spread_odds, home_projection, away_projection, home_team_score, away_team_score)
    select nm_away.clean_name || ' at ' || nm_home.clean_name || ' ' || g.start_time,
    'espn', g.over_under, g.over_odds, g.under_odds,
    gt_home.moneyline, gt_away.moneyline, abs(gt_home.spread), gt_home.spread_odds, gt_away.spread_odds,
    gt_home.game_projection, gt_away.game_projection, gt_home.fgm * 2 + gt_home._3pm + gt_home.ftm, gt_away.fgm * 2 + gt_away._3pm + gt_away.ftm
    from espn.games g
    join espn.game_teams gt_home on g.game_id = gt_home.game_id and gt_home.home = true
    join espn.game_teams gt_away on g.game_id = gt_away.game_id and gt_away.home = false
    join espn.teams t_home on t_home.team_id = gt_home.team_id
    join espn.teams t_away on t_away.team_id = gt_away.team_id
    join summary.name_mappings nm_home on nm_home.source = 'espn' and nm_home.source_id = t_home.team_id
    join summary.name_mappings nm_away on nm_away.source = 'espn'  and nm_away.source_id = t_away.team_id
    on conflict (game, source) do update set
    over_under = excluded.over_under,
    over_odds = excluded.over_odds,
    under_odds = excluded.under_odds,
    home_moneyline = excluded.home_moneyline,
    away_moneyline = excluded.away_moneyline,
    spread = excluded.spread,
    home_spread_odds = excluded.home_spread_odds,
    away_spread_odds = excluded.away_spread_odds,
    home_team_score = excluded.home_team_score,
    away_team_score = excluded.away_team_score,
    home_projection = excluded.home_projection,
    away_projection = excluded.away_projection;
    ";

    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();

    let q = "
    with events as (
        select event_id,
           max(case when market_type = 'TOTAL_POINTS_(OVER/UNDER)' then market_id end) as over_under_market,
           max(case when market_type = 'MONEY_LINE' then market_id end) as moneyline_market,
           max(case when market_type = 'MATCH_HANDICAP_(2-WAY)' then market_id end) as spread_market,
           max(market_time) as market_time
        from fanduel.markets
        where update_date >= current_timestamp - interval '1 hour'
        group by 1
    )
    insert into summary.games (game, source, over_under, over_odds, under_odds, home_moneyline, away_moneyline,
                               spread, home_spread_odds, away_spread_odds, home_projection, away_projection)
    select nm_away.clean_name || ' at ' || nm_home.clean_name || ' ' || replace((e.market_time::timestamp - interval '5 hour')::date::varchar, '-', ''),
           'fanduel', r_over.handicap, r_over.american_odds_int, r_under.american_odds_int,
           rm_home.american_odds_int, rm_away.american_odds_int, abs(rs_home.handicap), rs_home.american_odds_int, rs_away.american_odds_int,
           null, null
    from events e
    left join fanduel.runners r_over on r_over.market_id = e.over_under_market and r_over.result_type = 'OVER'
    left join fanduel.runners r_under on r_under.market_id = e.over_under_market and r_under.result_type = 'UNDER'
    left join fanduel.runners rm_home on rm_home.market_id = e.moneyline_market and rm_home.result_type = 'HOME'
    left join fanduel.runners rm_away on rm_away.market_id = e.moneyline_market and rm_away.result_type = 'AWAY'
    left join fanduel.runners rs_home on rs_home.market_id = e.spread_market and rs_home.result_type = 'HOME'
    left join fanduel.runners rs_away on rs_away.market_id = e.spread_market and rs_away.result_type = 'AWAY'
    left join summary.name_mappings nm_home on nm_home.source = 'fanduel' and (nm_home.source_name = rs_home.runner_name or nm_home.source_abbreviation = rs_home.runner_abbreviation or (nm_home.source_abbreviation is not null and nm_home.source_abbreviation = rs_home.runner_name))
    left join summary.name_mappings nm_away on nm_away.source = 'fanduel'  and (nm_away.source_name = rs_away.runner_name or nm_away.source_abbreviation = rs_away.runner_abbreviation or (nm_away.source_abbreviation is not null and nm_away.source_abbreviation = rs_away.runner_name))
    on conflict (game, source) do update set
    over_under = excluded.over_under,
    over_odds = excluded.over_odds,
    under_odds = excluded.under_odds,
    home_moneyline = excluded.home_moneyline,
    away_moneyline = excluded.away_moneyline,
    spread = excluded.spread,
    home_spread_odds = excluded.home_spread_odds,
    away_spread_odds = excluded.away_spread_odds,
    home_projection = excluded.home_projection,
    away_projection = excluded.away_projection;
    ";

    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();

    let q = "
    insert into summary.games (game, source, over_under, over_odds, under_odds, home_moneyline, away_moneyline,
        spread, home_spread_odds, away_spread_odds, home_projection, away_projection)
    select coalesce(sg_1home.game, sg_2home.game),
    'kenpom', null, null, null,
    null, null, null, null, null,
    round((case when sg_1home.game is not null then g.team_1_projection else g.team_2_projection end)*100.0, 3),
    round((case when sg_1home.game is not null then g.team_2_projection else g.team_1_projection end)*100.0, 3)
    from kenpom.fanmatch_games g
    join kenpom.teams t_one on t_one.team = g.team_1
    join kenpom.teams t_two on t_two.team = g.team_2
    join summary.name_mappings nm_one on nm_one.source = 'kenpom' and nm_one.source_name = t_one.team
    join summary.name_mappings nm_two on nm_two.source = 'kenpom'  and nm_two.source_name = t_two.team
    left join summary.games sg_2home on sg_2home.game = nm_one.clean_name || ' at ' || nm_two.clean_name || ' ' || replace(g.game_date, '-', '') and sg_2home.source = 'espn'
    left join summary.games sg_1home on sg_1home.game = nm_two.clean_name || ' at ' || nm_one.clean_name || ' ' || replace(g.game_date, '-', '') and sg_1home.source = 'espn'
    on conflict (game, source) do update set
    over_under = excluded.over_under,
    over_odds = excluded.over_odds,
    under_odds = excluded.under_odds,
    home_moneyline = excluded.home_moneyline,
    away_moneyline = excluded.away_moneyline,
    spread = excluded.spread,
    home_spread_odds = excluded.home_spread_odds,
    away_spread_odds = excluded.away_spread_odds,
    home_projection = excluded.home_projection,
    away_projection = excluded.away_projection;
    ";
    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();

    let q = "
    insert into summary.games (game, source, over_under, over_odds, under_odds, home_moneyline, away_moneyline,
        spread, home_spread_odds, away_spread_odds, home_projection, away_projection)
    select coalesce(sg_1home.game, sg_2home.game),
        'bart', null, null, null,
        null, null, null, null, null,
        case when sg_1home.game is null then g.home_team_projection else g.away_team_projection end,
        case when sg_1home.game is null then g.away_team_projection else g.home_team_projection end
    from bart.games g
    join bart.teams home_team on home_team.team = g.home_team
    join bart.teams away_team on away_team.team = g.away_team
    join summary.name_mappings nm_home on nm_home.source = 'bart' and nm_home.source_name = home_team.team
    join summary.name_mappings nm_away on nm_away.source = 'bart'  and nm_away.source_name = away_team.team
    left join summary.games sg_2home on sg_2home.game = nm_away.clean_name || ' at ' || nm_home.clean_name || ' ' || replace(g.game_date, '-', '') and sg_2home.source = 'espn'
    left join summary.games sg_1home on sg_1home.game = nm_home.clean_name || ' at ' || nm_away.clean_name || ' ' || replace(g.game_date, '-', '') and sg_1home.source = 'espn'
    on conflict (game, source) do update set
    over_under = excluded.over_under,
    over_odds = excluded.over_odds,
    under_odds = excluded.under_odds,
    home_moneyline = excluded.home_moneyline,
    away_moneyline = excluded.away_moneyline,
    spread = excluded.spread,
    home_spread_odds = excluded.home_spread_odds,
    away_spread_odds = excluded.away_spread_odds,
    home_projection = excluded.home_projection,
    away_projection = excluded.away_projection;
    ";
    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();


    let q = "
    insert into summary.name_mappings (clean_name, name_type, source, source_name, source_id, source_abbreviation)
    select distinct n.clean_name, 'team', 'vegas', coalesce(l.team_1_name, l2.team_2_name), null, coalesce(l.team_1_abbreviation, l2.team_2_abbreviation)
    from summary.names n
    left join vegas.lines l on clean_team_name(
        l.team_1_name
        ) = n.clean_name
    left join vegas.lines l2 on clean_team_name(
        l2.team_2_name
        ) = n.clean_name
    where l.team_1_name is not null or l2.team_2_name is not null
    on conflict (clean_name, name_type, source) do nothing;
    ";

    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();

    let q = "
    with events as (
        select team_1_id || '-' || lines.team_2_id as game_id,
           max(case when team_1_name <> 'Over' then team_1_name end) as team_1_name,
           max(case when team_2_name <> 'Under' then team_2_name end) as team_2_name,
            max(team_1_id) as team_1_id,
           max(team_2_id) as team_2_id,
            max(case when team_1_abbreviation <> 'Over' then team_1_abbreviation end) as team_1_abbreviation,
           max(case when team_2_abbreviation <> 'Under' then team_2_abbreviation end) as team_2_abbreviation,
           max(game_time) as game_time
        from vegas.lines
        group by 1
    )
    insert into summary.games (game, source, over_under, over_odds, under_odds, home_moneyline, away_moneyline,
                               spread, home_spread_odds, away_spread_odds, home_projection, away_projection)
    select
            coalesce(sg_1home.game, sg_2home.game),
           'vegas' || '-' || v.sportsbook,
           max(case when v.line_type = 'total' then v.team_1_handicap end),
           max(case when v.line_type = 'total' then v.team_1_odds end),
           max(case when v.line_type = 'total' then v.team_2_odds end),
           max(case when v.line_type = 'moneyline' and sg_1home.game is not null then v.team_1_odds when v.line_type = 'moneyline' and sg_1home.game is null then v.team_2_odds end),
           max(case when v.line_type = 'moneyline' and sg_1home.game is null then v.team_1_odds when v.line_type = 'moneyline' and sg_1home.game is not null then v.team_2_odds end),
           max(case when v.line_type = 'spread' then abs(v.team_1_handicap) end),
           max(case when v.line_type = 'spread' and sg_1home.game is not null then v.team_1_odds when v.line_type = 'spread' and sg_1home.game is null then v.team_2_odds end),
           max(case when v.line_type = 'spread' and sg_1home.game is null then v.team_1_odds when v.line_type = 'spread' and sg_1home.game is not null then v.team_2_odds end),
           null, null
    from events e
    join vegas.lines v on e.game_id = v.team_1_id || '-' || v.team_2_id
    left join summary.name_mappings nm_1 on nm_1.source = 'vegas' and (nm_1.source_name = e.team_1_name or nm_1.source_abbreviation = e.team_1_abbreviation)
    left join summary.name_mappings nm_2 on nm_2.source = 'vegas'  and (nm_2.source_name = e.team_2_name or nm_2.source_abbreviation = e.team_2_abbreviation)
    left join summary.games sg_2home on sg_2home.game = nm_1.clean_name || ' at ' || nm_2.clean_name || ' ' || replace((e.game_time::timestamp - interval '5 hour')::date::varchar, '-', '') and sg_2home.source = 'espn'
    left join summary.games sg_1home on sg_1home.game = nm_2.clean_name || ' at ' || nm_1.clean_name || ' ' || replace((e.game_time::timestamp - interval '5 hour')::date::varchar, '-', '') and sg_1home.source = 'espn'
    group by 1, v.sportsbook
    on conflict (game, source) do update set
    over_under = excluded.over_under,
    over_odds = excluded.over_odds,
    under_odds = excluded.under_odds,
    home_moneyline = excluded.home_moneyline,
    away_moneyline = excluded.away_moneyline,
    spread = excluded.spread,
    home_spread_odds = excluded.home_spread_odds,
    away_spread_odds = excluded.away_spread_odds,
    home_projection = excluded.home_projection,
    away_projection = excluded.away_projection;
    ";

    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();

    let q = "
    update summary.games
    set home_team = split_part(game, ' ', 1),
        away_team = split_part(game, ' ', 3),
        game_date = split_part(game, ' ', 4)
    ;
    ";
    let _result = sqlx::query(q).execute(&state.pool).await.unwrap();

    "Success"
}