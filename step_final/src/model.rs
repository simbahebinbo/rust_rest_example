use serde::{Deserialize, Serialize};

#[crud_table]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
  pub id: u64,
  pub name: String,
}

