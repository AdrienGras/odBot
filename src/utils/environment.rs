use anyhow::{bail, Result};

pub fn load() -> Result<()> {
    if let Err(error) = dotenv::from_filename(".env.local") {
        bail!(error);
    }

    Ok(())
}
