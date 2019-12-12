use super::frames::process::{InputProcess, OutputProcess};
use super::frames::Algorithm;
use crate::frames::FrameSimulator;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub no_frames: usize,
    pub no_processes: usize,
    pub algorithm: String,
    pub interval: usize,
    pub processes: Vec<InputProcess>,
}

#[derive(Debug, Serialize)]
pub struct OutputRequest {
    pub state: String,
    pub no_frames: usize,
    pub no_processes: usize,
    pub algorithm: String,
    pub avg_page_miss: f64,
    pub interval: usize,
    pub processes: Vec<OutputProcess>,
}

impl Request {
    pub fn get_algorithm(&self) -> Result<Algorithm, String> {
        use super::frames::Algorithm::*;

        match self.algorithm.as_ref() {
            "fifo" => Ok(Fifo),
            "lru" => Ok(Lru),
            "alru" => Ok(Alru),
            "rand" => Ok(Rand),
            "opt" => Ok(Opt),
            &_ => Err(String::from("Error, algorithm is not specified properly")),
        }
    }
}

impl OutputRequest {
    pub fn new_error(s: String, r: &Request) -> Self {
        OutputRequest {
            state: s,
            no_frames: r.no_frames,
            no_processes: r.no_processes,
            algorithm: r.algorithm.clone(),
            avg_page_miss: 0.0_f64,
            interval: r.interval,
            processes: Vec::new(),
        }
    }

    pub fn new_good(sim: &FrameSimulator, r: &Request) -> Self {
        OutputRequest {
            state: String::from("Good"),
            no_frames: r.no_frames,
            no_processes: r.no_processes,
            algorithm: r.algorithm.clone(),
            avg_page_miss: sim.get_avg_page_miss(),
            interval: r.interval,
            processes: sim.get_results(),
        }
    }

    pub fn error(s: String) -> Self {
        OutputRequest {
            state: s,
            no_processes: 0,
            no_frames: 0,
            algorithm: String::new(),
            avg_page_miss: 0.0_f64,
            interval: 0,
            processes: Vec::new(),
        }
    }
}
