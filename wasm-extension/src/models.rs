use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Problem {
    pub id: String,
    pub contest_id: String,
}
impl Problem {
    pub fn new(id: &str) -> Result<Self, &'static str> {
        // Check if the ID contains an underscore
        match id.rfind('_') {
            Some(pos) => {
                // Extract contest_id (everything before the last underscore)
                let contest_id = &id[0..pos];

                Ok(Self {
                    id: id.to_string(),
                    contest_id: contest_id.to_string(),
                })
            }
            None => Err("Invalid ID format: underscore not found"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
}
impl User {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}
