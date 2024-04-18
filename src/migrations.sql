create schema if not exists espn;

CREATE FUNCTION refresh_update_date() RETURNS TRIGGER
    LANGUAGE plpgsql
    AS $$
BEGIN
  NEW.update_date := current_timestamp;
  RETURN NEW;
END;
$$;

create table if not exists espn.games (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    game_id varchar unique not null,
    start_time timestamp
);

create or replace trigger trigger_espn_games_update_date
BEFORE UPDATE ON espn.games
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists espn.teams (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    team_id varchar unique not null,
    uid varchar,
    slug varchar,
    location varchar,
    name varchar,
    abbreviation varchar,
    display_name varchar,
    short_display_name varchar,
    color varchar,
    logo varchar
);

create or replace trigger trigger_espn_teams_update_date
BEFORE UPDATE ON espn.teams
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists espn.game_teams (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    team_id varchar not null,
    game_id varchar not null,
    fgm integer,
    fga integer,
    _3pm integer,
    _3pa integer,
    ftm integer,
    fta integer,
    reb integer,
    oreb integer,
    assists integer,
    steals integer,
    blocks integer,
    turnovers integer,
    techs integer,
    flagrants integer,
    turnover_points integer,
    fast_break_points integer,
    points_in_paint integer,
    fouls integer,
    largest_lead integer
);

create or replace trigger trigger_espn_game_teams_update_date
BEFORE UPDATE ON espn.game_teams
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();



create table if not exists espn.officials (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    name varchar unique
);

create or replace trigger trigger_espn_officials_update_date
BEFORE UPDATE ON espn.officials
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();


create table if not exists espn.game_officials (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    game_id varchar not null,
    name varchar not null,
    unique (game_id, name)
);

create or replace trigger trigger_espn_game_officials_update_date
BEFORE UPDATE ON espn.game_officials
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();



create table if not exists espn.venues (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    venue_id varchar unique not null,
    name varchar not null,
    city varchar,
    state varchar,
    capacity integer
);

create or replace trigger trigger_espn_venues_update_date
BEFORE UPDATE ON espn.venues
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists espn.game_venues (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    venue_id varchar not null,
    game_id varchar not null,
    attendance integer,
    unique(game_id, venue_id)
);

create or replace trigger trigger_espn_game_venues_update_date
BEFORE UPDATE ON espn.game_venues
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();


create table if not exists espn.players (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    player_id varchar unique not null,
    uid varchar,
    guid varchar,
    display_name varchar,
    short_name varchar,
    position varchar
);

create or replace trigger trigger_espn_players_update_date
BEFORE UPDATE ON espn.players
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists espn.game_players (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    player_id varchar not null,
    game_id varchar not null,
    team_id varchar not null,
    started bool,
    played bool,
    ejected bool,
    min integer,
    fgm integer,
    fga integer,
    _3pm integer,
    _3pa integer,
    ftm integer,
    fta integer,
    reb integer,
    oreb integer,
    assists integer,
    steals integer,
    blocks integer,
    turnovers integer,
    fouls integer,
    points integer,
    unique(game_id, player_id, team_id)
);

create or replace trigger trigger_espn_game_players_update_date
BEFORE UPDATE ON espn.game_players
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists espn.plays (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    play_id varchar unique not null,
    game_id varchar not null,
    sequence_number varchar,
    play_type varchar,
    play_text varchar,
    away_score integer,
    home_score integer,
    period integer,
    clock numeric,
    scoring_play boolean,
    score_value integer,
    team_id varchar,
    wall_clock varchar,
    shooting_play boolean
);

create or replace trigger trigger_espn_plays_update_date
BEFORE UPDATE ON espn.plays
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists espn.play_participants (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    play_id varchar not null,
    player_id varchar not null,
    unique(play_id, player_id)
);

create or replace trigger trigger_espn_play_participants_update_date
BEFORE UPDATE ON espn.play_participants
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

alter table espn.game_teams add column if not exists home boolean;
alter table espn.game_teams add column if not exists game_projection numeric;
alter table espn.game_teams add column if not exists favorite boolean;
alter table espn.game_teams add column if not exists underdog boolean;
alter table espn.game_teams add column if not exists moneyline integer;
alter table espn.game_teams add column if not exists spread numeric;
alter table espn.game_teams add column if not exists spread_odds integer;
alter table espn.game_teams add column if not exists opening_moneyline integer;
alter table espn.game_teams add column if not exists opening_spread numeric;
alter table espn.game_teams add column if not exists opening_spread_odds integer;

alter table espn.games add column if not exists over_under numeric;
alter table espn.games add column if not exists over_odds integer;
alter table espn.games add column if not exists under_odds integer;
alter table espn.games add column if not exists opening_over_under numeric;
alter table espn.games add column if not exists opening_over_odds integer;
alter table espn.games add column if not exists opening_under_odds integer;
alter table espn.games add column if not exists neutral_site boolean;

alter table espn.plays drop column if exists clock;
alter table espn.plays add column if not exists clock_minutes integer;
alter table espn.plays add column if not exists clock_seconds integer;


create table if not exists odds_updates (
    id serial primary key,
    create_date timestamp default current_timestamp,
    table_name varchar not null,
    column_name varchar not null,
    row_id integer not null,
    previous_value numeric,
    new_value numeric
);


create or replace function espn_game_teams_odds_update_fn() returns trigger language plpgsql as
$$
declare
  k text;
  v text;
  j_new jsonb := to_jsonb(new);
  j_old jsonb := to_jsonb(old);
begin
    if TG_OP = 'UPDATE' then
        for k, v in select * from jsonb_each_text(j_new) loop
            if ((v <> j_old ->> k) and k in ('moneyline', 'spread', 'spread_odds', 'opening_moneyline', 'opening_spread', 'opening_spread_odds')) then
                insert into odds_updates (table_name, column_name, row_id, previous_value, new_value)
                values (TG_TABLE_NAME, k, (j_new ->> 'id')::integer, (j_old ->> k)::numeric, v::numeric);
            end if;
         end loop;
    end if;
    return null;
end;
$$;
LANGUAGE 'plpgsql';

create or replace trigger odds_update_espn_game_teams_trigger
after update on espn.game_teams
for each row execute procedure espn_game_teams_odds_update_fn();

create or replace function espn_games_odds_update_fn() returns trigger language plpgsql as
$$
declare
  k text;
  v text;
  j_new jsonb := to_jsonb(new);
  j_old jsonb := to_jsonb(old);
begin
    if TG_OP = 'UPDATE' then
        for k, v in select * from jsonb_each_text(j_new) loop
            if ((v <> j_old ->> k) and k in ('over_under', 'over_odds', 'under_odds', 'opening_over_under', 'opening_over_odds', 'opening_under_odds')) then
                insert into odds_updates (table_name, column_name, row_id, previous_value, new_value)
                values (TG_TABLE_NAME, k, (j_new ->> 'id')::integer, (j_old ->> k)::numeric, v::numeric);
            end if;
         end loop;
    end if;
    return null;
end;
$$;
LANGUAGE 'plpgsql';

create or replace trigger odds_update_espn_games_trigger
after update on espn.games
for each row execute procedure espn_games_odds_update_fn();


create table if not exists fanduel.markets (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    market_id varchar unique not null,
    event_id integer,
    market_type varchar,
    market_status varchar,
    market_time varchar,
    betting_type varchar,
    market_name varchar
);

CREATE or replace TRIGGER trigger_fanduel_markets_update_date
BEFORE UPDATE ON fanduel.markets
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();


create table if not exists fanduel.runners (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    market_id varchar not null,
    selection_id integer,
    handicap numeric,
    runner_name varchar,
    runner_abbreviation varchar,
    result_type varchar,
    runner_status varchar,
    american_odds numeric,
    american_odds_int integer,
    unique(market_id, selection_id)
);

CREATE or replace TRIGGER trigger_fanduel_runners_update_date
BEFORE UPDATE ON fanduel.runners
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create or replace function fanduel_runners_odds_update_fn() returns trigger language plpgsql as
$$
declare
  k text;
  v text;
  j_new jsonb := to_jsonb(new);
  j_old jsonb := to_jsonb(old);
begin
    if TG_OP = 'UPDATE' then
        for k, v in select * from jsonb_each_text(j_new) loop
            if ((v <> j_old ->> k) and k in ('handicap', 'american_odds', 'american_odds_int')) then
                insert into odds_updates (table_name, column_name, row_id, previous_value, new_value)
                values (TG_TABLE_NAME, k, (j_new ->> 'id')::integer, (j_old ->> k)::numeric, v::numeric);
            end if;
         end loop;
    end if;
    return null;
end;
$$;
LANGUAGE 'plpgsql';

create or replace trigger odds_update_fanduel_runners_trigger
after update on fanduel.runners
for each row execute procedure fanduel_runners_odds_update_fn();

ALTER TABLE espn.games ALTER COLUMN start_time TYPE varchar;

create schema if not exists summary;

create table if not exists summary.names (
    id serial primary key,
    create_date timestamp default current_timestamp,
    clean_name varchar not null,
    name_type varchar not null,
    unique (clean_name, name_type)
);

create table if not exists summary.name_mappings (
    id serial primary key,
    create_date timestamp default current_timestamp,
    clean_name varchar not null,
    name_type varchar not null,
    source varchar not null,
    source_name varchar,
    source_id varchar,
    unique(clean_name, name_type, source)
);


create table if not exists summary.games (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    game varchar not null,
    source varchar not null,
    over_under numeric,
    over_odds integer,
    under_odds integer,
    home_moneyline integer,
    away_moneyline integer,
    spread numeric,
    home_spread_odds integer,
    away_spread_odds integer,
    home_projection numeric,
    away_projection numeric,
    unique (game, source)
);

CREATE or replace TRIGGER trigger_summary_games_update_date
BEFORE UPDATE ON summary.games
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create schema if not exists kenpom;

create table if not exists kenpom.teams (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    team varchar not null unique,
    conference varchar not null,
    wins integer not null,
    losses integer not null,
    adj_em numeric not null,
    adj_o numeric not null,
    adj_d numeric not null,
    adj_t numeric not null,
    luck numeric not null,
    sos_adj_em numeric not null,
    sos_adj_o numeric not null,
    sos_adj_d numeric not null,
    nc_sos_adj_em numeric not null
);

CREATE or replace TRIGGER trigger_kenpom_teams_update_date
BEFORE UPDATE ON kenpom.teams
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists kenpom.fanmatch_games (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    game_date varchar not null,
    team_1 varchar not null,
    team_1_rank varchar,
    team_1_projected_score integer,
    team_1_projection numeric,
    team_2 varchar not null,
    team_2_rank varchar,
    team_2_projected_score integer,
    team_2_projection numeric,
    game_location varchar,
    game_venue varchar,
    thrill_score numeric,
    comeback integer,
    excitement numeric,
    unique(game_date, team_1, team_2)
);

CREATE or replace TRIGGER trigger_kenpom_fanmatch_games_update_date
BEFORE UPDATE ON kenpom.fanmatch_games
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create schema if not exists bart;

create table if not exists bart.teams (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    team varchar not null unique,
    conference varchar not null,
    wins integer not null,
    losses integer not null,
    adjoe numeric not null,
    adjde numeric not null,
    barthag numeric not null,
    efg numeric not null,
    efgd numeric not null,
    tor numeric not null,
    tord numeric not null,
    orb numeric not null,
    drb numeric not null,
    ftr numeric not null,
    ftrd numeric not null,
    _2p numeric not null,
    _2pd numeric not null,
    _3p numeric not null,
    _3pd numeric not null,
    adjt numeric not null,
    wb numeric not null
);

CREATE or replace TRIGGER trigger_bart_teams_update_date
BEFORE UPDATE ON bart.teams
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists bart.games (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    game_date varchar not null,
    home_team varchar not null,
    away_team varchar not null,
    neutral_site bool not null,
    home_team_projected_score integer,
    home_team_projection numeric,
    home_team_rank integer,
    away_team_projected_score integer,
    away_team_projection numeric,
    away_team_rank integer,
    thrill_score numeric,
    unique(game_date, home_team, away_team)
);

CREATE or replace TRIGGER trigger_bart_games_update_date
BEFORE UPDATE ON bart.games
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

alter table bart.games add column if not exists spread numeric;

create table if not exists bart.players (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    player_name varchar not null,
    team varchar not null,
    conf varchar,
    gp integer,
    min_per numeric,
    o_rtg numeric,
    usg numeric,
    e_fg numeric,
    ts_per numeric,
    orb_per numeric,
    drb_per numeric,
    ast_per numeric,
    to_per numeric,
    ftm integer,
    fta integer,
    ft_per numeric,
    two_pm integer,
    two_pa integer,
    two_p_per numeric,
    tpm integer,
    tpa integer,
    tp_per numeric,
    blk_per numeric,
    stl_per numeric,
    ftr numeric,
    yr varchar,
    ht varchar,
    num integer,
    porpag numeric,
    adjoe numeric,
    unknown_1 numeric,
    year integer,
    pid integer,
    type varchar,
    rec_rank numeric,
    ast_tov numeric,
    rimmade integer,
    rimmade_plus_rimmiss integer,
    midmade integer,
    midmade_plus_midmiss numeric,
    rim_perc numeric,
    mid_perc numeric,
    dunksmade integer,
    dunksmade_plus_dunksmiss integer,
    dunks_perc numeric,
    pick numeric,
    drtg numeric,
    adrtg numeric,
    dporpag numeric,
    stops numeric,
    bpm numeric,
    obpm numeric,
    dbpm numeric,
    gbpm numeric,
    mp numeric,
    ogbpm numeric,
    dgbpm numeric,
    oreb numeric,
    dreb numeric,
    treb numeric,
    ast numeric,
    stl numeric,
    blk numeric,
    pts numeric,
    position varchar,
    unknown_2 numeric,
    unique(player_name, team)
);

CREATE or replace TRIGGER trigger_bart_players_update_date
BEFORE UPDATE ON bart.players
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create schema if not exists donbest;

create table if not exists donbest.injuries (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    team_id varchar not null,
    player_id varchar not null,
    team_name varchar not null,
    player_first_name varchar,
    player_last_name varchar,
    injury varchar,
    start_date varchar,
    status varchar,
    display_status varchar,
    note varchar,
    last_updated varchar,
    unique(team_id, player_id, player_first_name, player_last_name, team_name, injury)
);

CREATE or replace TRIGGER trigger_donbest_injuries_update_date
BEFORE UPDATE ON donbest.injuries
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

create table if not exists injury_updates (
    id serial primary key,
    create_date timestamp default current_timestamp,
    table_name varchar not null,
    column_name varchar not null,
    row_id integer not null,
    previous_value varchar,
    new_value varchar
);



create or replace function donbest_injury_update_fn() returns trigger language plpgsql as
$$
declare
  k text;
  v text;
  j_new jsonb := to_jsonb(new);
  j_old jsonb := to_jsonb(old);
begin
    if TG_OP = 'UPDATE' then
        for k, v in select * from jsonb_each_text(j_new) loop
            if ((v <> j_old ->> k) and k in ('start_date', 'status', 'display_status', 'note', 'last_updated')) then
                insert into injury_updates (table_name, column_name, row_id, previous_value, new_value)
                values (TG_TABLE_NAME, k, (j_new ->> 'id')::integer, (j_old ->> k), v);
            end if;
         end loop;
    end if;
    return null;
end;
$$;
LANGUAGE 'plpgsql';

create or replace trigger donbest_injury_update_trigger
after update on donbest.injuries
for each row execute procedure donbest_injury_update_fn();

alter table summary.name_mappings add column if not exists source_abbreviation varchar;

alter table summary.games add column if not exists home_team_score integer;
alter table summary.games add column if not exists away_team_score integer;

ALTER TABLE bart.players ALTER COLUMN num type varchar;

ALTER TABLE bart.players ADD CONSTRAINT bart_players_player_team_year_unique_idx UNIQUE (player_name, team, year);

ALTER TABLE bart.players DROP CONSTRAINT players_player_name_team_key;


create schema if not exists twitter;

create table if not exists twitter.tweets (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    user_id varchar not null,
    user_name varchar not null,
    user_screen_name varchar not null,
    full_text varchar not null,
    tweet_id varchar,
    tweet_time varchar,
    unique(user_id, tweet_id)
);

CREATE or replace TRIGGER trigger_twitter_tweets_update_date
BEFORE UPDATE ON twitter.tweets
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();


create schema if not exists vegas;

create table if not exists vegas.lines (
    id serial primary key,
    create_date timestamp default current_timestamp,
    update_date timestamp default current_timestamp,
    game_time varchar not null,
    team_1_id varchar not null,
    team_1_name varchar not null,
    team_1_abbreviation varchar,
    team_2_id varchar not null,
    team_2_name varchar not null,
    team_2_abbreviation varchar,
    sportsbook varchar not null,
    line_type varchar not null,
    team_1_handicap numeric,
    team_1_odds integer,
    team_2_handicap numeric,
    team_2_odds integer,
    unique(team_1_id, team_2_id, sportsbook, game_time, line_type)
);

CREATE or replace TRIGGER trigger_vegas_lines_update_date
BEFORE UPDATE ON vegas.lines
FOR EACH ROW
EXECUTE PROCEDURE refresh_update_date();

alter table summary.games add column if not exists home_team varchar;
alter table summary.games add column if not exists away_team varchar;
alter table summary.games add column if not exists game_date varchar;