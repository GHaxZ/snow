use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use crate::{objects::ObjectManager, terrain::TerrainManager};

pub struct Renderer {
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn init() -> Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::Clear(ClearType::All)
        )?;

        let (width, height) = terminal::size()?;
        Ok(Self { width, height })
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn draw_scene(&self, terrain: &TerrainManager, objects: &ObjectManager) -> Result<()> {
        execute!(io::stdout(), terminal::Clear(ClearType::All))?;

        self.draw_ground(terrain)?;
        self.draw_hills(terrain)?;
        self.draw_objects(objects)?;

        Ok(())
    }

    fn draw_ground(&self, terrain: &TerrainManager) -> Result<()> {
        execute!(
            io::stdout(),
            cursor::MoveTo(0, self.height - terrain.ground_height())
        )?;
        write!(io::stdout(), "{}", terrain.ground_content())?;
        Ok(())
    }

    fn draw_hills(&self, terrain: &TerrainManager) -> Result<()> {
        for (x, col) in terrain.hills_content().iter().enumerate() {
            for (y, ch) in col.chars().enumerate() {
                execute!(
                    io::stdout(),
                    cursor::MoveTo(x as u16, self.height - terrain.ground_height() - y as u16)
                )?;
                write!(io::stdout(), "{}", ch)?;
            }
        }
        Ok(())
    }

    fn draw_objects(&self, objects: &ObjectManager) -> Result<()> {
        for (obj_type, (x, y)) in objects.get_positions() {
            let content = obj_type.content();

            for (i, line) in content.iter().enumerate() {
                let current_y = *y + i as u16;

                if current_y >= self.height {
                    continue;
                }

                let end_x = *x + line.len() as u16;
                if *x >= self.width {
                    continue;
                }

                let line = if end_x > self.width {
                    &line[..(self.width - x) as usize]
                } else {
                    line
                };

                if current_y <= self.height {
                    execute!(io::stdout(), cursor::MoveTo(*x, current_y))?;
                    write!(io::stdout(), "{}", line)?;
                }
            }
        }
        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(
            io::stdout(),
            terminal::LeaveAlternateScreen,
            cursor::Show,
            terminal::Clear(ClearType::All)
        )?;
        Ok(())
    }
}
