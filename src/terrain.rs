use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::cmp::Ordering;

pub struct TerrainManager {
    ground: Ground,
    hills: Hills,
    snowfall: Snowfall,
}

pub struct Snowflake {
    symbol: char,
    x: i32,
    y: u16,
}

impl Snowflake {
    fn new(x: i32, y: u16) -> Self {
        Self {
            symbol: random_snowflake(),
            x,
            y,
        }
    }

    pub fn position(&self) -> (u16, u16) {
        (self.x as u16, self.y)
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }
}

struct Snowfall {
    flakes: Vec<Snowflake>,
    width: u16,
    height: u16,
    flake_density: f32,
}

impl Snowfall {
    pub fn new(width: u16, height: u16, flake_density: f32) -> Self {
        Self {
            flakes: Vec::new(),
            width,
            height,
            flake_density: flake_density.clamp(0.0, 1.0),
        }
    }

    fn random_x() -> i32 {
        rand::thread_rng().gen_range(-1..2)
    }

    fn flake_chance(&self) -> bool {
        let rand = rand::thread_rng().gen_range(0.0..1.0 as f32);

        rand <= self.flake_density
    }

    fn update_dimensions(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    fn update_snowflakes(&mut self) {
        // Remove all out of bounds snowflakes
        self.flakes
            .retain(|f| f.y <= self.height && (0..(self.width + 1) as i32).contains(&f.x));

        // Move all snowflakes
        for flake in self.flakes.iter_mut() {
            flake.y += 1;
            flake.x += Self::random_x()
        }

        // Spawn new snowflakes
        for i in 0..self.width {
            if self.flake_chance() {
                self.flakes.push(Snowflake::new(i as i32, 0));
            }
        }
    }
}

impl TerrainManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            ground: Ground::new(width, Self::calc_terrain_height(height)),
            hills: Hills::new(width, Self::calc_terrain_height(height), 0.02, 0.6),
            snowfall: Snowfall::new(width, height, 0.15),
        }
    }

    fn calc_terrain_height(screen_height: u16) -> u16 {
        (screen_height as f32 * 0.3) as u16
    }

    pub fn height(&self) -> u16 {
        self.ground.height() + self.hills.height()
    }

    pub fn hills(&self) -> &Hills {
        &self.hills
    }

    pub fn snowflakes(&self) -> &Vec<Snowflake> {
        &self.snowfall.flakes
    }

    pub fn regenerate(&mut self, width: u16, height: u16) {
        let terrain_height = Self::calc_terrain_height(height);
        self.ground = Ground::new(width, terrain_height);
        self.hills = Hills::new(width, terrain_height, 0.02, 0.6);
        self.snowfall = Snowfall::new(width, height, 0.15)
    }

    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        let terrain_height = Self::calc_terrain_height(height);
        self.ground.update_dimensions(width, terrain_height);
        self.hills.update_dimensions(width, terrain_height);
        self.snowfall.update_dimensions(width, height);
    }

    pub fn update_snow(&mut self) {
        self.snowfall.update_snowflakes()
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
            str.push(random_snowflake());
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
                    self.content.push(random_snowflake());
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

fn random_snowflake() -> char {
    let num: u32 = rand::thread_rng().gen_range(0..3);
    match num {
        0 => '.',
        1 => '+',
        _ => '*',
    }
}
