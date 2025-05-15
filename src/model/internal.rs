use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Positions {
    pub jungle: Vec<usize>,
    pub top: Vec<usize>,
    pub middle: Vec<usize>,
    pub bottom: Vec<usize>,
    pub support: Vec<usize>,
}

pub type MetaItems = HashMap<String, Positions>;
