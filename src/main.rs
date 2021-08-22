use color_eyre::eyre::{eyre, Result};
use rand::seq::SliceRandom;

#[derive(Clone, Debug, serde::Deserialize)]
struct Eatery {
    name: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let eateries = csv::Reader::from_path("eatery.csv")?
        .into_deserialize()
        .collect::<Result<Vec<Eatery>, csv::Error>>()?;
    let eatery = eateries
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| eyre!("failed to get random eatery"))?;
    println!("{}", eatery.name);
    Ok(())
}
