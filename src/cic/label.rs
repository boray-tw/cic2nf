#[derive(Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct Label {
    index: u8, // 0 for no index, 1 for benign
    name: String,
}

impl Label {
    pub fn new(index: u8, name: String) -> Self {
        Self { index, name }
    }

    pub fn new_with_name(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }

    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn set_index_mut(&mut self, index: u8) {
        self.index = index;
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
