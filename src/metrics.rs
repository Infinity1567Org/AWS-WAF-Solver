use rand::Rng; // For generating random numbers
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    fp2: usize,
    browser: usize,
    capabilities: usize,
    gpu: usize,
    dnt: usize,
    math: usize,
    screen: usize,
    navigator: usize,
    auto: usize,
    stealth: usize,
    subtle: usize,
    canvas: usize,
    formdetector: usize,
    be: usize,
}

impl Metrics {
    pub fn new() -> Metrics {
        let mut rng = rand::thread_rng();
        Metrics {
            fp2: 1,
            browser: 0,
            capabilities: 0,
            gpu: rng.gen_range(13..=16), 
            dnt: 0,
            math: 0,
            screen: 0,
            navigator: 0,
            auto: 0,
            stealth: 1,
            subtle: 0,
            canvas: rng.gen_range(13..=16), // Generates a value between 13 and 16
            formdetector: 0,
            be: 0,
        }
    }
}



