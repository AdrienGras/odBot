use std::{vec, collections::HashMap};

use anyhow::{Ok, Result};

use log::debug;
use serenity::model::user::User;

use crate::{
    core::application_context::ApplicationContext,
    models::{
        babyfoot_match,
        babyfoot_match_quote, babyfoot_stat,
        person::{self, Person},
    }, renderers::babyfoot_renderer,
};

pub struct BabyfootMiddleware<'a> {
    app: &'a ApplicationContext,
}

impl<'a> BabyfootMiddleware<'a> {
    pub fn new(app: &'a ApplicationContext) -> Self {
        Self { app }
    }

    pub async fn register_match_1v1(
        &self,
        j1: &User,
        j2: &User,
        score_j1: i32,
        score_j2: i32,
    ) -> Result<String> {
        let db = &self.app.db;

        debug!("Checking J1 in database...");
        let p1: Person = person::get_or_create_person_from_discord_user(db, j1).await?;

        debug!("Checking J2 in database...");
        let p2: Person = person::get_or_create_person_from_discord_user(db, j2).await?;

        debug!("Registering match...");
        let created = babyfoot_match::create_match_from_players_and_scores(
            db,
            vec![(vec![p1.clone()], score_j1), (vec![p2.clone()], score_j2)],
        )
        .await?;

        debug!("Creating stats for J1...");
        babyfoot_stat::create(db, &p1, score_j1, score_j1 > score_j2).await?;
        debug!("Creating stats for J2...");
        babyfoot_stat::create(db, &p2, score_j2, score_j1 < score_j2).await?;

        debug!("Creating register_match response...");
        let quote = babyfoot_match_quote::random(db).await?;
        let message = babyfoot_renderer::render_match(&created, quote)?;

        Ok(message)
    }

    pub async fn register_match_2v2(
        &self,
        j1: &User,
        j2: &User,
        j3: &User,
        j4: &User,
        score_j1j2: i32,
        score_j3j4: i32,
    ) -> Result<String> {
        let db = &self.app.db;

        debug!("Checking J1 in database...");
        let p1: Person = person::get_or_create_person_from_discord_user(db, j1).await?;

        debug!("Checking J2 in database...");
        let p2: Person = person::get_or_create_person_from_discord_user(db, j2).await?;

        debug!("Checking J3 in database...");
        let p3: Person = person::get_or_create_person_from_discord_user(db, j3).await?;

        debug!("Checking J2 in database...");
        let p4: Person = person::get_or_create_person_from_discord_user(db, j4).await?;

        debug!("Registering match...");
        let created = babyfoot_match::create_match_from_players_and_scores(
            db,
            vec![(vec![p1.clone(), p2.clone()], score_j1j2), (vec![p2.clone(), p3.clone()], score_j3j4)],
        )
        .await?;

        debug!("Creating stats for J1...");
        babyfoot_stat::create(db, &p1, score_j1j2, score_j1j2 > score_j3j4).await?;
        debug!("Creating stats for J2...");
        babyfoot_stat::create(db, &p2, score_j1j2, score_j1j2 > score_j3j4).await?;
        debug!("Creating stats for J3...");
        babyfoot_stat::create(db, &p3, score_j3j4, score_j1j2 < score_j3j4).await?;
        debug!("Creating stats for J4...");
        babyfoot_stat::create(db, &p4, score_j3j4, score_j1j2 < score_j3j4).await?;

        debug!("Creating register_match response...");
        let quote = babyfoot_match_quote::random(db).await?;
        let message = babyfoot_renderer::render_match(&created, quote)?;

        Ok(message)
    }

    pub async fn register_match_1v2(
        &self,
        j1: &User,
        j3: &User,
        j4: &User,
        score_j1: i32,
        score_j3j4: i32,
    ) -> Result<String> {
        let db = &self.app.db;

        debug!("Checking J1 in database...");
        let p1: Person = person::get_or_create_person_from_discord_user(db, j1).await?;

        debug!("Checking J3 in database...");
        let p3: Person = person::get_or_create_person_from_discord_user(db, j3).await?;

        debug!("Checking J2 in database...");
        let p4: Person = person::get_or_create_person_from_discord_user(db, j4).await?;

        debug!("Registering match...");
        let created = babyfoot_match::create_match_from_players_and_scores(
            db,
            vec![(vec![p1.clone()], score_j1), (vec![p3.clone(), p4.clone()], score_j3j4)],
        )
        .await?;

        debug!("Creating stats for J1...");
        babyfoot_stat::create(db, &p1, score_j1, score_j1 > score_j3j4).await?;
        debug!("Creating stats for J3...");
        babyfoot_stat::create(db, &p3, score_j3j4, score_j1 < score_j3j4).await?;
        debug!("Creating stats for J4...");
        babyfoot_stat::create(db, &p4, score_j3j4, score_j1 < score_j3j4).await?;

        debug!("Creating register_match response...");
        let quote = babyfoot_match_quote::random(db).await?;
        let message = babyfoot_renderer::render_match(&created, quote)?;

        Ok(message)
    }


    pub async fn player_stat(&self, player: &User) -> Result<String> {
        let db = &self.app.db;

        debug!("Checking player in database...");
        let p: Person = person::get_or_create_person_from_discord_user(db, player).await?;

        let all_stats = babyfoot_stat::find_all_by_person(db, &p).await?;

        let total_matches = all_stats.len();
        let mut nb_pts: i32 = 0;
        let mut nb_win: i32 = 0;
        let mut nb_lose: i32 = 0;

        for stat in all_stats.iter() {
            nb_pts = nb_pts + stat.score;

            if stat.is_winning {
                nb_win = nb_win + 1;
            } else {
                nb_lose = nb_lose + 1;
            }
        }

        let message = babyfoot_renderer::render_player_stats(&p, total_matches.try_into()?, nb_pts, nb_win, nb_lose)?;

        Ok(message)
    }

    pub async fn leaderboards(&self) -> Result<String> {
        let db = &self.app.db;

        let mut pts_map: HashMap<String, i32> = HashMap::new();
        let mut win_map: HashMap<String, i32> = HashMap::new();

        let all_stats = babyfoot_stat::all(db).await?;

        for s in all_stats.iter() {
            let key: String = s.player_id.clone();

            let previous_pts = if pts_map.contains_key(&key) {*pts_map.get(&key).unwrap()} else {0};
            pts_map.insert(key.clone(), previous_pts + s.score);

            if s.is_winning {
                let previous_win = if win_map.contains_key(&key) {*win_map.get(&key).unwrap()} else {0};
                win_map.insert(key.clone(), previous_win + 1);    
            }
        }

        let mut pts_vec: Vec<(i32, String)> = Vec::new();
        let mut win_vec: Vec<(i32, String)> = Vec::new();

        for (p, s) in pts_map.iter() {
            pts_vec.push((*s, p.clone()));
        }

        for (p, s) in win_map.iter() {
            win_vec.push((*s, p.clone()));
        }

        pts_vec.sort_by(|a, b| {
            a.0.cmp(&b.0)
        });
        win_vec.sort_by(|a, b| {
            a.0.cmp(&b.0)
        });

        pts_vec.reverse();
        win_vec.reverse();

        Ok(babyfoot_renderer::render_leaderboards(pts_vec, win_vec)?)
    }
}
