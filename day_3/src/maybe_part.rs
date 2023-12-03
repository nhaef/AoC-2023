pub struct MaybePart {
    is_part: bool,
    number: u32,
}

impl MaybePart {
    pub fn new(number: u32) -> Self {
        Self {
            is_part: false,
            number,
        }
    }
    pub fn mark_as_real(&mut self) {
        self.is_part = true;
    }
    pub fn is_marked_as_real(&self) -> bool {
        self.is_part
    }
    pub fn get_number(&self) -> u32 {
        self.number
    }
}

impl From<MaybePart> for u32 {
    fn from(value: MaybePart) -> Self {
        value.number
    }
}