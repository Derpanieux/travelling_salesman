use std::fmt;
pub struct City {
    pub x: f32,
    pub y: f32,
}
impl Eq for City{}
impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}
impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Point at {}, {}", self.x, self.y);
    }
}
impl City {
    pub fn dist(&mut self, other: &City) -> f32 {
        let deltax = self.x - other.x;
        let deltay = self.x - other.y;
        return (deltax*deltax+deltay*deltay).sqrt();
    }
}