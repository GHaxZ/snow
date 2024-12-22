use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::{cmp::Ordering, collections::HashMap};

pub trait Object {
    fn width() -> u16;
    fn height() -> u16;
    fn content() -> &'static str;
    fn offset() -> u16 {
        Self::width() / 2 + 3
    }
}

pub struct Snowman {}
pub struct House {}
pub struct Tree {}
pub struct Snowflake {}

pub struct Snowfall {
    flakes: HashMap<Snowflake, (u16, u16)>,
    width: u16,
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
            str.push_str(Snowflake::content());
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

    // Helper method to get height at specific x position
    pub fn height_at(&self, x: u16) -> u16 {
        if x >= self.width {
            return 0;
        }
        self.content[x as usize].chars().count() as u16
    }
}

// Ground implementation remains unchanged
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
                    self.content.push_str(Snowflake::content());
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

// Original Object implementations remain unchanged
impl Object for Snowflake {
    fn width() -> u16 {
        1
    }
    fn height() -> u16 {
        1
    }
    fn content() -> &'static str {
        let num: u32 = rand::thread_rng().gen_range(0..3);
        match num {
            0 => ".",
            1 => "+",
            _ => "*",
        }
    }
}

impl Object for Snowman {
    fn width() -> u16 {
        9
    }
    fn height() -> u16 {
        4
    }
    fn content() -> &'static str {
        r#"  _==_ _
_,(",)|_|
 \/. \-|
 ( :  )|"#
    }
}

impl Object for House {
    fn width() -> u16 {
        14
    }
    fn height() -> u16 {
        7
    }
    fn content() -> &'static str {
        r#"       `'::.
  _________H
 /\     _   \
/  \___/^\___\
|  | []   [] |
|  |   .-.   |
@._|@@_|||_@@|"#
    }
}

impl Object for Tree {
    fn width() -> u16 {
        29
    }
    fn height() -> u16 {
        11
    }
    fn content() -> &'static str {
        r#"        \/ |    |/
      \/ / \||/  /_/___/_
       \/   |/ \/
  _\__\_\   |  /_____/_
         \  | /          /
__ _-----`  |{,-----------~
          \ }{
           }{{
           }}{
           {{}
        ,=~{}{-_"#
    }
}
