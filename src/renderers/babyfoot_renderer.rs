use anyhow::Result;
use serenity::{utils::MessageBuilder, model::prelude::UserId};

use crate::models::{babyfoot_match::BabyfootMatch, person::Person};

use super::utils_renderer;

pub fn render_saved_match() -> Result<String> {
    let mb = &mut MessageBuilder::new();

    mb.push("💾 Match enregistré !");

    Ok(mb.build())
}

pub fn render_match(m: &BabyfootMatch, quote: String) -> Result<String> {
    // render match result
    let match_result_render = &mut MessageBuilder::new();

    for (i, p) in m.team_1.players.iter().enumerate() {
        if i > 0 {
            match_result_render.push(", ");
        }
        match_result_render.mention(&UserId(p.discord_id.parse()?));
    }

    match_result_render.push("\t");
    match_result_render.push_bold(m.score_team_1);
    match_result_render.push("\t-\t");
    match_result_render.push_bold(m.score_team_2);
    match_result_render.push("\t");

    for (i, p) in m.team_2.players.iter().enumerate() {
        if i > 0 {
            match_result_render.push(", ");
        }
        match_result_render.mention(&UserId(p.discord_id.parse()?));
    }

    match_result_render.build();

    // creating person list
    let mut players = m.team_1.players.to_vec();
    let mut players_team_2 = m.team_2.players.to_vec();
    players.append(&mut players_team_2);

    let player_list_mb = &mut MessageBuilder::new();
    for (i, p) in players.iter().enumerate() {
        if i > 0 {
            player_list_mb.push(", ");
        }
        player_list_mb.mention(&UserId(p.discord_id.parse()?));
    }
    let player_list = player_list_mb.build();

    // render message
    let message = MessageBuilder::new()
    .quote_rest()
    .push_bold_line("Match")
    .push_bold_line("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    .push_bold(format!("Date:\t"))
    .push_line(m.date.format("%v %R"))
    .push_bold(format!("Joueurs\t"))
    .push_line(player_list)
    .push_bold_line("Résultats:\t")
    .push_line("_ _")
    .push_line(match_result_render)
    .push_line("_ _")
    .push_line(
        MessageBuilder::new()
            .push("⚽ 🥳")
            .push_bold(format!(
                "       {}",
                quote
            ))
            .build(),
    )
    .push_line("_ _")
    .build();

    Ok(message)
}

pub fn render_player_stats(person: &Person, nb_total_match: i32, nb_pts: i32, nb_win: i32, nb_lose: i32) -> Result<String> {
    let discord_user = UserId(person.discord_id.parse()?);
    let wl_ratio: f32 = (nb_win * 100 / nb_total_match) as f32;

    let message = MessageBuilder::new()
        .quote_rest()
        .push_bold_line("Statistiques")
        .push_bold_line("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
        .push_bold("Joueur:\t")
        .mention(&discord_user)
        .push_line("")
        .push_bold("Nb de matchs:\t")
        .push_line(nb_total_match)
        .push_bold("Nb de points:\t")
        .push_line(nb_pts)
        .push_bold("Win/Lose:\t")
        .push_line(format!("{} / {}\t({}%)", nb_win, nb_lose, wl_ratio.round()))
        .build();
    
    Ok(message)
}

pub fn render_leaderboards(pts: Vec<(i32, String)>, wins: Vec<(i32, String)>) -> Result<String> {

    let message = &mut MessageBuilder::new();

    message
        .quote_rest()
        .push_bold_line("Leaderboard - Meilleur buteur")
        .push_bold_line("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for (i, p) in pts.iter().enumerate() {
        let nb_string = utils_renderer::leaderboard_place_symbol(i)?;

        message
            .push(nb_string)
            .push(" ")
            .mention(&UserId(p.1.parse()?))
            .push("\t")
            .push_line(format!("({} pts)", p.0));
    }

    message.push_line("_ _");

    message
        .push_bold_line("Leaderboard - Nb de wins")
        .push_bold_line("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for (i, p) in wins.iter().enumerate() {
        let nb_string = utils_renderer::leaderboard_place_symbol(i)?;

        message
            .push(nb_string)
            .push(" ")
            .mention(&UserId(p.1.parse()?))
            .push("\t")
            .push_line(format!("({} wins)", p.0));
    }

    Ok(message.build())
}