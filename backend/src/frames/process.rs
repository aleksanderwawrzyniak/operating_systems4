use super::memory::step::Step;

#[derive(Debug, Deserialize, Clone)]
pub struct InputProcess {
    pub id: usize,
    pub size: usize,
    pub requests: String,
}

#[derive(Debug, Serialize)]
pub struct OutputProcess {
    pub id: usize,
    pub size: usize,
    pub faults: u64,
    pub fault_percentage: f64,
    pub no_frames: usize,
    pub steps: Vec<Step>,
}
