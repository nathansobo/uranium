use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
pub struct Point {
    pub row: i32,
    pub column: i32
}

impl Point {
    pub fn new(row: i32, column: i32) -> Point {
        Point {row: row, column: column}
    }

    pub fn zero() -> Point {
        Point {row: 0, column: 0}
    }

    pub fn extent(text: &str) -> Point {
        let mut row = 0;
        let mut column = 0;

        for c in text.chars() {
            if c == '\n' { row += 1 } else { column += 1 };
        }

        Point::new(row, column)
    }

    pub fn is_zero(&self) -> bool {
        self.row == 0 && self.column == 0
    }

    pub fn traverse(&self, distance: &Point) -> Point {
        if distance.row == 0 {
            Point::new(self.row, self.column + distance.column)
        } else {
            Point::new(self.row + distance.row, distance.column)
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.column.cmp(&other.column),
            Ordering::Greater => Ordering::Greater
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn ordering() {
        assert!(Point::new(0, 0) == Point::new(0, 0));
        assert!(Point::new(0, 0) < Point::new(1, 0));
        assert!(Point::new(0, 0) < Point::new(0, 1));
        assert!(Point::new(1, 0) > Point::new(0, 0));
        assert!(Point::new(0, 1) > Point::new(0, 0));
    }

    #[test]
    fn traversal() {
        assert_eq!(Point::new(0, 5).traverse(&Point::new(0, 10)), Point::new(0, 15));
        assert_eq!(Point::new(0, 10).traverse(&Point::new(0, -5)), Point::new(0, 5));
        assert_eq!(Point::new(0, 5).traverse(&Point::new(2, 10)), Point::new(2, 10));
        assert_eq!(Point::new(2, 10).traverse(&Point::new(-2, 5)), Point::new(0, 5));
    }
}
