#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Contest {
    pub id: String,
    pub start_epoch_second: u64,
    pub duration_second: u64,
    pub title: String,
    pub rate_change: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Submission {
    pub id: u64,
    pub epoch_second: u64,
    pub problem_id: String,
    pub contest_id: String,
    pub user_id: String,
    pub language: String,
    pub point: f64,
    pub length: u64,
    pub result: String,
    pub execution_time: Option<u64>,
}
impl Submission {
    pub fn is_accepted(&self) -> bool {
        self.result == "AC"
    }
}
