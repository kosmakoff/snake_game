#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Offset {
    pub delta_x: i32,
    pub delta_y: i32,
}

impl Point {
    pub fn offset<O: Into<Offset>>(self, offset: O) -> Self {
        let offset: Offset = offset.into();

        Point {
            x: self.x + offset.delta_x,
            y: self.y + offset.delta_y,
        }
    }
}

impl From<&Point> for Point {
    fn from(point: &Point) -> Point {
        (*point).clone()
    }
}

impl From<Point> for Offset {
    fn from(point: Point) -> Offset {
        Offset {
            delta_x: point.x,
            delta_y: point.y
        }
    }
}

define_conversions! {
    Point, i32, i32, [x, y];
    Point, u32, i32, [x, y];
    Offset, i32, i32, [delta_x, delta_y];
    Offset, u32, i32, [delta_x, delta_y];
}