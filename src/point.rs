#[derive(Copy, Clone, Default)]
pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn divide(&mut self, divsor: f32) {
        self.x = self.x / divsor;
        self.y = self.y / divsor;
    }

    pub fn add(&mut self, point: Point) {
        self.x = self.x + point.x();
        self.y = self.y + point.y();
    }
}

impl Eq for Point {
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x() &&
            self.y == other.y()
    }

    fn ne(&self, other: &Point) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let point : Point = Default::default();
        assert_eq!(0f32, point.x());
        assert_eq!(0f32, point.y());
    }

    #[test]
    fn test_new() {
        let x = 1f32;
        let y = 2f32;
        let point = Point::new(x, y);
        assert_eq!(x, point.x());
        assert_eq!(y, point.y());
    }

    #[test]
    fn test_set_x() {
        let x = 1f32;
        let y = 2f32;
        let mut point = Point::new(x, y);
        point.set_x(3f32); 
        assert_eq!(3f32, point.x())
    }

    #[test]
    fn test_set_y() {
        let x = 1f32;
        let y = 2f32;
        let mut point = Point::new(x, y);
        point.set_y(3f32); 
        assert_eq!(3f32, point.y())
    }

    #[test]
    fn test_equal() {
        let x = 1f32;
        let y = 2f32;
        let point1 = Point::new(x, y);
        let point2 = Point::new(x, y);
        assert!(point1.eq(&point2))
    }

    #[test]
    fn test_nequal() {
        let x = 1f32;
        let y = 2f32;
        let point1 = Point::new(x, y);
        let point2 = Point::new(x, y + 0.1f32);
        assert!(point1.ne(&point2))
    }

    #[test]
    fn test_divide() {
        let x = 10f32;
        let y = 20f32;
        let mut point = Point::new(x, y);
        point.divide(2f32);
        assert_eq!(5f32, point.x());
        assert_eq!(10f32, point.y());
    }

    #[test]
    fn test_add() {
        let x = 10f32;
        let y = 20f32;
        let mut point = Point::new(x, y);
        point.add(Point::new(-1f32, -3f32));
        assert_eq!(9f32, point.x());
        assert_eq!(17f32, point.y());
    }
}
