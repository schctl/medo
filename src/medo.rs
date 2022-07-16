use crate::Group;

pub struct Medo {
    pub groups: Vec<Group>,
}

impl Medo {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }
}
