// domain/hello.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub value: String
}
