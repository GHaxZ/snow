mod app;
mod objects;
mod renderer;
mod terrain;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    App::run()?;

    Ok(())
}
