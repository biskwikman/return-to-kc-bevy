use bevy::prelude::*;
// pub struct Rect {
//     pub x1: i32,
//     pub x2: i32,
//     pub y1: i32,
//     pub y2: i32,
// }

pub trait Intersect {
    fn is_intersect(&self, other: Rect) -> bool;
}

impl Intersect for Rect {
    // pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
    //     Rect {
    //         x1: x,
    //         y1: y,
    //         x2: x + w,
    //         y2: y + h,
    //     }
    // }

    // fn intersect(&self, other: Rect) -> bool {
    //     self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.x2 >= other.y1
    // }

    fn is_intersect(&self, other: Rect) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    // pub fn center(&self) -> (i32, i32) {
    //     ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    // }
}
