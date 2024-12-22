use std::collections::HashMap;

use crate::terrain::TerrainManager;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum ObjectType {
    Snowman,
    Tree,
    House,
}

impl ObjectType {
    pub fn content(&self) -> &str {
        match self {
            ObjectType::Snowman => SNOWMAN_SPRITE,
            ObjectType::Tree => TREE_SPRITE,
            ObjectType::House => HOUSE_SPRITE,
        }
    }

    pub fn width(&self) -> u16 {
        match self {
            ObjectType::Snowman => 9,
            ObjectType::Tree => 24,
            ObjectType::House => 14,
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            ObjectType::Snowman => 4,
            ObjectType::Tree => 11,
            ObjectType::House => 7,
        }
    }

    pub fn offset(&self) -> u16 {
        self.width() / 2
    }
}

pub struct ObjectManager {
    positions: HashMap<ObjectType, (u16, u16)>,
    initialized: bool,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            initialized: false,
        }
    }

    pub fn reset(&mut self) {
        self.initialized = false;
        self.positions.clear();
    }

    pub fn place_objects(&mut self, terrain: &TerrainManager) {
        if self.initialized {
            return;
        }

        let (lowest_x, lowest_h) = terrain.get_lowest_point();
        let snowman_pos = lowest_x - ObjectType::Snowman.offset();
        let snowman_y = terrain.screen_height()
            - terrain.ground_height()
            - lowest_h
            - ObjectType::Snowman.height()
            + 2;
        self.positions
            .insert(ObjectType::Snowman, (snowman_pos, snowman_y));

        let (highest_x, highest_h) = terrain.get_highest_point();
        let tree_pos = highest_x - ObjectType::Tree.offset();
        let tree_y = terrain.screen_height()
            - terrain.ground_height()
            - highest_h
            - ObjectType::Tree.height()
            + 2;
        self.positions.insert(ObjectType::Tree, (tree_pos, tree_y));

        self.initialized = true;
    }

    pub fn get_positions(&self) -> &HashMap<ObjectType, (u16, u16)> {
        &self.positions
    }
}

#[rustfmt::skip]
const SNOWMAN_SPRITE: &str = r#"  _==_ _
_,(",)|_|
 \/. \-|
 ( :  )|"#;

const TREE_SPRITE: &str = r#"        \/ |    |/
      \/ / \||/  /_/___/_
       \/   |/ \/,
  _\__\_\   |  /_____/_
         \  | /          /
__ _-----`  |{,-----------~
          \ }{
           }{{
           }}{
           {{}
        ,=~{}{-_"#;

const HOUSE_SPRITE: &str = r#"       `'::.
  _________H
 /\     _   \
/  \___/^\___\
|  | []   [] |
|  |   .-.   |
@._|@@_|||_@@|"#;