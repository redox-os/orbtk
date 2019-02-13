/// Used to align a widget vertical or horizontal.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Alignment {
    /// Calculates the position (x or y) of the widget depending on the available measure, the goal measure
    /// margin and alignment.
    pub fn align_position(
        &self,
        available_measure: f64,
        measure: f64,
        margin_start: f64,
        margin_end: f64,
    ) -> f64 {
        match self {
            Alignment::End => available_measure - measure - margin_end,
            Alignment::Center => (available_measure - measure) / 2.0,
            _ => margin_start,
        }
    }

    /// Calculates the measure (measure or height) of the widget depending on the available measure, the goal measure
    /// margin and horizontal alignment.
    pub fn align_measure(
        &self,
        available_measure: f64,
        measure: f64,
        margin_start: f64,
        margin_end: f64,
    ) -> f64 {
        match self {
            Alignment::Stretch => available_measure - margin_start - margin_end,
            _ => measure,
        }
    }
}

impl From<&str> for Alignment {
    fn from(t: &str) -> Self {
        match t {
            "End" | "end" => Alignment::End,
            "Center" | "center" => Alignment::Center,
            "Start" | "start" => Alignment::Start,
            _ => Alignment::Stretch,
        }
    }
}
