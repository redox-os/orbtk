
pub enum Alignment {
    Horizontal,
    Vertical,
}

impl Alignment {
    pub fn major(&self, coords: (u32, u32)) -> u32 {
        match *self {
            Alignment::Horizontal => coords.0,
            Alignment::Vertical => coords.1,
        }
    }

    pub fn minor(&self, coords: (u32, u32)) -> u32 {
        match *self {
            Alignment::Horizontal => coords.1,
            Alignment::Vertical => coords.0,
        }
    }

    pub fn pack(&self, major: u32, minor: u32) -> (u32, u32) {
        match *self {
            Alignment::Horizontal => (major, minor),
            Alignment::Vertical => (minor, major),
        }
    }
}