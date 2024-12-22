use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::cmp::Ordering;

pub struct TerrainManager {
    ground: Ground,
    hills: Hills,
}

impl TerrainManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            ground: Ground::new(width, Self::calc_terrain_height(height)),
            hills: Hills::new(width, Self::calc_terrain_height(height), 0.02, 0.6),
        }
    }

    fn calc_terrain_height(screen_height: u16) -> u16 {
        (screen_height as f32 * 0.3) as u16
    }

    pub fn screen_height(&self) -> u16 {
        self.ground.height() * 10 / 3
    }

    pub fn regenerate(&mut self, width: u16, height: u16) {
        let terrain_height = Self::calc_terrain_height(height);
        self.ground = Ground::new(width, terrain_height);
        self.hills = Hills::new(width, terrain_height, 0.02, 0.6);
    }

    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        let terrain_height = Self::calc_terrain_height(height);
        self.ground.update_dimensions(width, terrain_height);
        self.hills.update_dimensions(width, terrain_height);
    }

    pub fn ground_height(&self) -> u16 {
        self.ground.height()
    }

    pub fn ground_content(&self) -> &String {
        self.ground.content()
    }

    pub fn hills_content(&self) -> &Vec<String> {
        self.hills.content()
    }

    pub fn get_highest_point(&self) -> (u16, u16) {
        let highest_x = (0..self.hills.width())
            .max_by(|&x1, &x2| {
                self.hills
                    .real_height(x1)
                    .partial_cmp(&self.hills.real_height(x2))
                    .unwrap()
            })
            .unwrap_or(0);

        (highest_x, self.hills.display_height(highest_x))
    }

    pub fn get_lowest_point(&self) -> (u16, u16) {
        let lowest_x = (0..self.hills.width())
            .min_by(|&x1, &x2| {
                self.hills
                    .real_height(x1)
                    .partial_cmp(&self.hills.real_height(x2))
                    .unwrap()
            })
            .unwrap_or(0);

        (lowest_x, self.hills.display_height(lowest_x))
    }
}

pub struct Hills {
    perlin: Perlin,
    width: u16,
    height: u16,
    x: u16,
    content: Vec<String>,
    frequency: f64,
    amplitude: f64,
    offset: f64,
}

impl Hills {
    pub fn new(width: u16, height: u16, frequency: f64, amplitude: f64) -> Self {
        let seed = rand::thread_rng().gen();

        let mut new = Self {
            perlin: Perlin::new(seed),
            width,
            height,
            x: 0,
            content: Vec::new(),
            frequency,
            amplitude,
            offset: 0.5,
        };

        new.update_content();
        new
    }

    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.update_content();
    }

    fn update_content(&mut self) {
        self.content.clear();
        self.x = 0;

        for _ in 0..self.width {
            self.content.push(self.generate_strip());
            self.x += 1;
        }
    }

    fn generate_strip(&self) -> String {
        let scaled_x = self.x as f64 * self.frequency;
        let raw_noise = self.perlin.get([scaled_x]);
        let normalized_noise = (raw_noise + 1.0) / 2.0;
        let scaled_height =
            normalized_noise * self.amplitude + (1.0 - self.amplitude) * self.offset;
        let height = ((self.height as f64 * scaled_height).max(1.0) as u16).min(self.height);

        let mut str = String::with_capacity(height as usize);
        for _ in 0..height {
            str.push_str(random_snowflake());
        }
        str
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn content(&self) -> &Vec<String> {
        &self.content
    }

    pub fn display_height(&self, x: u16) -> u16 {
        if x >= self.width {
            return 0;
        }
        self.content[x as usize].chars().count() as u16
    }

    pub fn real_height(&self, x: u16) -> f64 {
        let scaled_x = x as f64 * self.frequency;
        self.perlin.get([scaled_x])
    }
}

pub struct Ground {
    width: u16,
    height: u16,
    content: String,
}

impl Ground {
    pub fn new(width: u16, height: u16) -> Self {
        let mut new = Self {
            width,
            height,
            content: String::new(),
        };
        new.update_content();
        new
    }

    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.update_content();
    }

    fn update_content(&mut self) {
        let flake_count = (self.width * self.height) as usize;
        let content_len = self.content.chars().count();

        match content_len.cmp(&flake_count) {
            Ordering::Greater => {
                self.content.truncate(flake_count);
            }
            Ordering::Less => {
                for _ in 0..(flake_count - content_len) {
                    self.content.push_str(random_snowflake());
                }
            }
            Ordering::Equal => {}
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}

fn random_snowflake() -> &'static str {
    let num: u32 = rand::thread_rng().gen_range(0..3);
    match num {
        0 => ".",
        1 => "+",
        _ => "*",
    }
}
