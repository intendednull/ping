use std::rc::Rc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Address(Rc<Uuid>);

impl Address {
    pub fn new() -> Self {
        Self(Uuid::new_v4().into())
    }
}
