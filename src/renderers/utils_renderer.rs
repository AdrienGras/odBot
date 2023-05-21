use anyhow::Result;

pub fn leaderboard_place_symbol(idx: usize) -> Result<String> {
    let default_nb_string = format!("{}", idx+1);

    let nb_string = match idx {
        0 => "🥇",
        1 => "🥈",
        2 => "🥉",
        _ => default_nb_string.as_str()
    };

    Ok(nb_string.to_string())
}