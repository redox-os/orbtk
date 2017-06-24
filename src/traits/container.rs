// The idea behind this trait is that, at least for now
// layout can be handled by dividing the window rectangle up into
// smaller rectangles
use rect::Rect;

pub enum Side {
    Top,
    Bot,
    Lef,
    Rig,
}

fn percent_u32(whole_num: u32, percent: f64) -> Result<u32, &'static str> {
    if percent <= 100.0 && percent > 0.0 {
        let scaled_to_1e7 = percent * 100000.0;
        let divided = (((whole_num as f64) * scaled_to_1e7) / 10000000.0).floor();
        Ok(divided as u32)
    } else {
        Err("Invalid percent passed as an argument.")
    }
}


fn percent_i32(whole_num: u32, percent: f64) -> Result<i32, &'static str> {
    Ok(percent_u32(whole_num, percent)? as i32)
}

fn half_percent_i32(whole_num: u32, percent: f64) -> Result<i32, &'static str> {
    Ok(percent_i32(whole_num, percent)?/2)
}


pub trait Container {
    fn center(&self, percent: f64) -> Result<Rect, &'static str>;
    fn shave(&self, percent: f64, tblr: Side) -> Result<Rect, &'static str>;
}

impl Container for Rect {
    // If passed a rectangle and a number less than 100, center will return a rectangle
    // that is percent size the original and centered in the original.
    // Returns an error if the percent is not in the range 0 < percent <= 100 
    fn center(&self, percent: f64) -> Result<Rect, &'static str> {
        let out_of_100 = 100.0 - percent;
        Ok(Rect::new((self.x + half_percent_i32(self.width, out_of_100)?),
                  (self.y + half_percent_i32(self.height, out_of_100)?),
                  (self.width - percent_u32(self.width, out_of_100)?),
                  (self.height - percent_u32(self.height, out_of_100)?)))
    }

    // If passed a Rect, a number less than 100, and a side, shave will return
    // a rectangle chopped down on that side to the percent given if
    // the percent is in the range 0 < percent <= 100. Otherwise, returns
    // an error.
    fn shave(&self, percent: f64, tblr: Side) -> Result<Rect, &'static str> {
        use traits::container::Side::{Top, Bot, Lef, Rig};
        let out_of_100 = 100.0 - percent;
        match tblr {
            Top => { Ok(Rect::new(self.x,
                               (self.y + percent_i32(self.height, out_of_100)?),
                               self.width,
                               (self.height - percent_u32(self.height, out_of_100)?)))},
            Bot => { Ok(Rect::new(self.x, self.y, self.width,
                              (self.height - percent_u32(self.height, out_of_100)?)))},
            Lef => { Ok(Rect::new((self.x + percent_i32(self.width, out_of_100)?),
                               self.y,
                               (self.width - percent_u32(self.width, out_of_100)?),
                               self.height))},
            Rig => { Ok(Rect::new(self.x, self.y,
                                   (self.width - percent_u32(self.width, out_of_100)?),
                                   self.height))},
        }
    }
}
