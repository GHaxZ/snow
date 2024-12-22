mod app;
mod objects;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    App::run()?;

    Ok(())
}
