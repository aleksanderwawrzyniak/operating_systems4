#[derive(Debug, Serialize, Clone)]
pub struct Step {
    pub pages: String,
    pub algorithm_step: String,
    pub fault_rate: f64,
}

impl Step {
    pub fn filled_with(pages: String, algorithm_step: String, fault_rate: f64) -> Self {
        Step {
            pages,
            algorithm_step,
            fault_rate,
        }
    }
}
