use anyhow::Result;
use serenity::utils::MessageBuilder;

pub fn error() -> Result<String> {
    let mb = &mut MessageBuilder::new();

    mb.push("💥 Aie... y'a comme un bug non ?");

    Ok(mb.build())
}

pub fn success() -> Result<String> {
    let mb = &mut MessageBuilder::new();

    mb.push("✅ Commande exécutée !");

    Ok(mb.build())
}