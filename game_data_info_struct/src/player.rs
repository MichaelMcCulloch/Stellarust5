use serde_derive::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PlayerClass {
    Human(String),
    Computer,
}
