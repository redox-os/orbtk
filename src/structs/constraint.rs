#[derive(Clone, Copy)]
pub struct Constraint {
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}

impl Constraint {
    pub fn perform(&self, size: (u32, u32)) -> (u32, u32) {
        (
            constrain(size.0, self.min_width, self.max_width),
            constrain(size.1, self.min_height, self.max_height),
        )
    }
}

fn constrain(val: u32, min: u32, max: u32) -> u32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
