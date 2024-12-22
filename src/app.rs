use std::{io, io::Write, thread, time::Duration};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};

use crate::objects::{Ground, Hills, Object, Snowman, Tree};

const FPS: u64 = 60;

use std::collections::HashMap;

pub struct App {
    running: bool,
    frametime: u64,
    width: u16,
    height: u16,
    ground: Ground,
    hills: Hills,
    objects: HashMap<String, (u16, u16)>,
}

impl App {
    pub fn run() -> Result<()> {
        let mut app = Self::init()?;

        app.generate_landscape()?;

        while app.running {
            app.poll_events()?;
            thread::sleep(Duration::from_millis(app.frametime));
        }

        app.exit()
    }

    fn init() -> Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen)?;
        execute!(io::stdout(), cursor::Hide)?;
        execute!(io::stdout(), terminal::Clear(ClearType::All))?;

        let (w, h) = terminal::size()?;

        Ok(Self {
            running: true,
            frametime: 1000 / FPS,
            ground: Ground::new(w, (h as f32 * 0.3) as u16),
            hills: Hills::new(w, (h as f32 * 0.3) as u16, 0.02, 0.6),
            width: w,
            height: h,
            objects: HashMap::new(),
        })
    }

    fn generate_landscape(&mut self) -> Result<()> {
        self.ground
            .update_dimensions(self.width, (self.height as f32 * 0.3) as u16);
        self.hills = Hills::new(self.width, (self.height as f32 * 0.3) as u16, 0.02, 0.6); // Regenerate Perlin noise
        self.ground = Ground::new(self.width, (self.height as f32 * 0.3) as u16);

        self.generate_static_objects();

        self.draw_landscape()?;
        Ok(())
    }

    fn generate_static_objects(&mut self) {
        self.objects.clear();

        let lowest_point = (0..self.hills.width())
            .min_by_key(|&x| self.hills.height_at(x))
            .unwrap_or(0);

        let snowman_y = self.height - self.ground.height() - self.hills.height_at(lowest_point);

        self.objects.insert(
            "snowman".to_string(),
            (lowest_point + Snowman::offset(), snowman_y),
        );

        let highest_point = (0..self.hills.width())
            .max_by_key(|&x| self.hills.height_at(x))
            .unwrap_or(0);

        let tree_y = self.height - self.ground.height() - self.hills.height_at(highest_point);

        self.objects
            .insert("tree".to_string(), (highest_point - Tree::offset(), tree_y));
    }

    fn draw_landscape(&self) -> Result<()> {
        execute!(io::stdout(), terminal::Clear(ClearType::All))?;

        execute!(
            io::stdout(),
            cursor::MoveTo(0, self.height - self.ground.height())
        )?;
        write!(io::stdout(), "{}", self.ground.content())?;

        let hillcon = self.hills.content();
        for (x, str) in hillcon.iter().enumerate() {
            for (y, char) in str.chars().enumerate() {
                let cursor_position =
                    cursor::MoveTo(x as u16, self.height - self.ground.height() - y as u16);
                write!(io::stdout(), "{}{}", cursor_position, char)?;
            }
        }

        for (name, (x, y)) in &self.objects {
            let content = match name.as_str() {
                "snowman" => Snowman::content(),
                "tree" => Tree::content(),
                _ => continue,
            };

            for (i, line) in content.lines().rev().enumerate() {
                let start_x = *x;
                let end_x = start_x + line.len() as u16;

                if start_x >= self.width {
                    continue;
                }

                let truncated_line = if end_x > self.width {
                    let cutoff = (self.width - start_x) as usize;
                    &line[..cutoff]
                } else {
                    line
                };

                execute!(io::stdout(), cursor::MoveTo(start_x, y - i as u16))?;
                write!(io::stdout(), "{}", truncated_line)?;
            }
        }

        Ok(())
    }

    fn update_dimensions(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;

        self.ground
            .update_dimensions(width, (height as f32 * 0.3) as u16);
        self.hills
            .update_dimensions(width, (height as f32 * 0.3) as u16);

        self.draw_landscape().unwrap();
    }

    fn poll_events(&mut self) -> Result<()> {
        if let Ok(event) = event::poll(Duration::from_millis(self.frametime)) {
            if event {
                let event = event::read()?;

                match event {
                    Event::Key(key) => self.handle_key(key)?,
                    Event::Resize(w, h) => {
                        self.update_dimensions(w, h);
                        self.draw_landscape()?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => self.exit()?,
            KeyCode::Char('r') => {
                self.generate_landscape()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.running = false;

        terminal::disable_raw_mode()?;
        execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
        execute!(io::stdout(), cursor::Show)?;
        execute!(io::stdout(), terminal::Clear(ClearType::All))?;

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.exit().expect("Failed restoring terminal");
    }
}
