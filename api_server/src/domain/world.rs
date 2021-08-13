// domain/world.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub value: String
}
