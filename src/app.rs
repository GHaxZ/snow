//  FIX: Tree clips into floor after resize (probably a rounding error)

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{objects::ObjectManager, renderer::Renderer, terrain::TerrainManager};

const FPS: u64 = 60;
const SNOWFALL_DELTA_MS: u128 = 500;

pub struct App {
    running: bool,
    frametime: u64,
    renderer: Renderer,
    terrain_manager: TerrainManager,
    object_manager: ObjectManager,
}

impl App {
    pub fn run() -> Result<()> {
        let mut app = Self::init()?;
        app.generate_landscape()?;

        let mut timestamp = Instant::now();

        while app.running {
            app.poll_events()?;

            if timestamp.elapsed().as_millis() >= SNOWFALL_DELTA_MS {
                app.redraw()?;

                timestamp = Instant::now();
            }

            thread::sleep(Duration::from_millis(app.frametime));
        }

        app.exit()
    }

    fn init() -> Result<Self> {
        let renderer = Renderer::init()?;
        let (width, height) = renderer.dimensions();

        Ok(Self {
            running: true,
            frametime: 1000 / FPS,
            terrain_manager: TerrainManager::new(width, height),
            object_manager: ObjectManager::new(),
            renderer,
        })
    }

    fn generate_landscape(&mut self) -> Result<()> {
        let (width, height) = self.renderer.dimensions();

        self.terrain_manager.regenerate(width, height);
        self.object_manager.reset();
        self.object_manager
            .place_objects(&self.terrain_manager, &self.renderer);
        self.renderer
            .draw_scene(&self.terrain_manager, &self.object_manager)
    }

    fn poll_events(&mut self) -> Result<()> {
        if let Ok(event) = event::poll(Duration::from_millis(self.frametime)) {
            if event {
                match event::read()? {
                    Event::Key(key) => self.handle_key(key)?,
                    Event::Resize(w, h) => self.handle_resize(w, h)?,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_resize(&mut self, width: u16, height: u16) -> Result<()> {
        self.renderer.update_dimensions(width, height);
        self.terrain_manager.update_dimensions(width, height);
        self.object_manager
            .update_position(&self.terrain_manager, &self.renderer);

        self.redraw()?;

        Ok(())
    }

    fn redraw(&mut self) -> Result<()> {
        self.terrain_manager.update_snow();
        self.renderer
            .draw_scene(&self.terrain_manager, &self.object_manager)?;

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => self.exit()?,
            KeyCode::Char('r') => self.generate_landscape()?,
            _ => {}
        }

        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.running = false;
        self.renderer.cleanup()?;

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.exit().expect("Failed restoring terminal");
    }
}
