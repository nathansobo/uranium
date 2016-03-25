use point::Point;

pub struct Patch {
    root: Option<Box<Node>>
}

impl Patch {
    pub fn new() -> Patch {
        Patch {root: None}
    }

    pub fn splice_with_text(&self, start: &Point, old_extent: &Point, new_text: &str) {
        let new_extent = Point::extent(new_text);
    }
}

pub struct Node {
    parent: Option<Box<Node>>,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>
}
