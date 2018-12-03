#[derive(PartialEq, PartialOrd)]
pub struct Rectangle {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Rectangle {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> Rectangle {
        Rectangle {
            left: left,
            top: top,
            width: width,
            height: height,
        }
    }

    pub fn get_area(&self) -> usize {
        self.width * self.height
    }

    pub fn get_right(&self) -> usize {
        self.left + self.width
    }

    pub fn get_bottom(&self) -> usize {
        self.top + self.height
    }

    pub fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        let mut left = 0;
        let mut top = 0;
        let mut width = 0;
        let mut height = 0;

        //if other contains self on the horizontal plane
        if self.left >= other.left && self.left <= other.get_right() {
            left = self.left;

            if self.get_right() <= other.get_right() {
                width = self.width;
            } else {
                width = other.get_right() - self.left;
            }
        }
        //if self contains other on the horizontal plane
        else if other.left >= self.left && other.left <= self.get_right() {
            left = other.left;

            if other.get_right() <= self.get_right() {
                width = other.width;
            } else {
                width = self.get_right() - other.left;
            }
        }

        //if other contains self on the vertical plane
        if self.top >= other.top && self.top <= other.get_bottom() {
            top = self.top;

            if self.get_bottom() <= other.get_bottom() {
                height = self.height;
            } else {
                height = other.get_bottom() - self.top;
            }
        } else if other.top >= self.top && other.top <= self.get_bottom() {
            top = other.top;

            if other.get_bottom() <= self.get_bottom() {
                height = other.height;
            } else {
                height = self.get_bottom() - other.top;
            }
        }

        if width == 0 || height == 0 {
            return None;
        }

        Some(Rectangle::new(left, top, width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let r = Rectangle::new(0,0,2,3);
        let area = r.get_area();

        assert_eq!(area, 6);
    }

    #[test]
    fn test_right() {
        let r = Rectangle::new(0,0,2,3);
        let right = r.get_right();

        assert_eq!(right, 2);
    }

    #[test]
    fn test_bottom() {
        let r = Rectangle::new(0,0,2,3);
        let bottom = r.get_bottom();

        assert_eq!(bottom, 3);
    }

    #[test]
    fn test_intersect_none() {
        let r = Rectangle::new(0, 0, 2, 3);
        let r2 = Rectangle::new(5,5,2,3);

        let over = match r.intersect(&r2){
            Some(_) => true,
            None => false
        };

        assert_eq!(false, over);

        let over2 = match r2.intersect(&r){
            Some(_) => true,
            None => false
        };

        assert_eq!(false, over2);
    }

    #[test]
    fn test_intersect_left_partial() {
        let r = Rectangle::new(2, 2, 5, 5);
        let r2 = Rectangle::new(0,0,5,5);

        let over = r.intersect(&r2).unwrap();

        assert_eq!(2, over.left);
        assert_eq!(2, over.top);
        assert_eq!(3, over.width);
        assert_eq!(3, over.height);

        let over2 = r2.intersect(&r).unwrap();

        assert_eq!(2, over2.left);
        assert_eq!(2, over2.top);
        assert_eq!(3, over2.width);
        assert_eq!(3, over2.height);
    }

    #[test]
    fn test_intersect_left_full() {
        let r = Rectangle::new(2, 2, 2, 2);
        let r2 = Rectangle::new(0,0,5,5);

        let over = r.intersect(&r2).unwrap();

        assert_eq!(2, over.left);
        assert_eq!(2, over.top);
        assert_eq!(2, over.width);
        assert_eq!(2, over.height);

        let over2 = r2.intersect(&r).unwrap();

        assert_eq!(2, over2.left);
        assert_eq!(2, over2.top);
        assert_eq!(2, over2.width);
        assert_eq!(2, over2.height);
    }
}