use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use crate::{objects::ObjectManager, terrain::TerrainManager};

#[derive(Clone, Default)]
struct Cell {
    character: char,
}

pub struct Renderer {
    width: u16,
    height: u16,
    buffer: Vec<Vec<Cell>>,
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
        let buffer = vec![vec![Cell::default(); height as usize]; width as usize];

        Ok(Self {
            width,
            height,
            buffer,
        })
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.buffer = vec![vec![Cell::default(); height as usize]; width as usize];
    }

    fn write_to_buffer(&mut self, x: u16, y: u16, ch: char) {
        if x < self.width && y < self.height {
            self.buffer[x as usize][y as usize].character = ch;
        }
    }

    pub fn draw_scene(&mut self, terrain: &TerrainManager, objects: &ObjectManager) -> Result<()> {
        self.clear_buffer();

        self.draw_snow(terrain)?;
        self.draw_ground(terrain)?;
        self.draw_hills(terrain)?;
        self.draw_objects(objects)?;

        self.render_buffer()?;

        Ok(())
    }

    fn clear_buffer(&mut self) {
        for row in &mut self.buffer {
            for cell in row {
                cell.character = ' ';
            }
        }
    }

    fn render_buffer(&self) -> Result<()> {
        for (y, row) in (0..self.height as usize).map(|y| {
            (
                y,
                self.buffer
                    .iter()
                    .map(|col| col[y].character)
                    .collect::<String>(),
            )
        }) {
            execute!(io::stdout(), cursor::MoveTo(0, y as u16))?;
            write!(io::stdout(), "{}", row)?;
        }

        io::stdout().flush()?;

        Ok(())
    }

    fn draw_snow(&mut self, terrain: &TerrainManager) -> Result<()> {
        for flake in terrain.snowflakes() {
            let (x, y) = flake.position();
            self.write_to_buffer(x, y, flake.symbol());
        }

        Ok(())
    }

    fn draw_ground(&mut self, terrain: &TerrainManager) -> Result<()> {
        let ground_content = terrain.ground_content();

        for (row, line) in ground_content.iter().enumerate() {
            let y = self.height.saturating_sub(row as u16 + 1);
            for (x, ch) in line.chars().enumerate() {
                self.write_to_buffer(x as u16, y, ch);
            }
        }

        Ok(())
    }

    fn draw_hills(&mut self, terrain: &TerrainManager) -> Result<()> {
        let base_y = self.height - terrain.ground_height();

        for (x, col) in terrain.hills_content().iter().enumerate() {
            for (y, ch) in col.chars().enumerate() {
                if ch != ' ' {
                    let screen_y = base_y.saturating_sub(y as u16);
                    self.write_to_buffer(x as u16, screen_y, ch);
                }
            }
        }
        Ok(())
    }

    fn draw_objects(&mut self, objects: &ObjectManager) -> Result<()> {
        for (obj_type, (x, y)) in objects.get_positions() {
            let content = obj_type.content();

            for (i, line) in content.lines().rev().enumerate() {
                let current_y = y.saturating_sub(i as u16);
                for (j, ch) in line.chars().enumerate() {
                    let current_x = x + j as u16;
                    if ch != '°' {
                        self.write_to_buffer(current_x, current_y, ch);
                    }
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
