use serde::Deserialize;

#[derive(Deserialize)]
pub struct Positions {
    pub jungle: Vec<usize>,
    pub top: Vec<usize>,
    pub mid: Vec<usize>,
    pub adc: Vec<usize>,
    pub support: Vec<usize>,
}
