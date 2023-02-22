#[derive(Default, Debug)]
pub struct Process {
    id: String,
}

impl Process {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}
