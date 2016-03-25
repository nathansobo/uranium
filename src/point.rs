pub struct Point {
    pub row: i32,
    pub column: i32
}

impl Point {
    pub fn new(row: i32, column: i32) -> Point {
        Point {row: row, column: column}
    }

    pub fn extent(text: &str) -> Point {
        let mut row = 0;
        let mut column = 0;

        for c in text.chars() {
            if c == '\n' { row += 1 } else { column += 1 };
        }

        Point::new(row, column)
    }
}
