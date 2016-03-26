use std::rc::Rc;
use std::rc::Weak;
use point::Point;

pub struct Patch<'a> {
    root: Option<Rc<Node>>,
    cursor: Option<Cursor<'a>>
}

impl<'a> Patch<'a> {
    pub fn new() -> Patch<'a> {
        let mut patch = Patch {root: None, cursor: None};
        patch.cursor = Some(Cursor::new(&mut patch.root));
        patch
    }

    pub fn splice_with_text(&self, start: &Point, old_text: &str, new_text: &str) {
        let old_extent = Point::extent(old_text);
        let new_extent = Point::extent(new_text);
        self.splice(start, &old_extent, &new_extent, old_text, new_text);
    }

    pub fn splice(&self, start: &Point, old_extent: &Point, new_extent: &Point, old_text: &str, new_text: &str) {
        if old_extent.is_zero() && new_extent.is_zero() { return; }

        let output_old_end = start.traverse(old_extent);
        let output_new_end = start.traverse(new_extent);

        // let start_node = self.cursor.insert_splice_boundary(start);
    }
}

pub struct Node {
    parent: Option<Weak<Node>>,
    left_child: Option<Rc<Node>>,
    right_child: Option<Rc<Node>>
}

struct Cursor<'a> {
    current_node: &'a mut Option<Rc<Node>>,
    input_start: Point,
    output_start: Point,

    left_ancestor: Option<&'a Node>,
    left_ancestor_input_position: Option<Point>,
    left_ancestor_output_position: Option<Point>,
    left_ancestor_stack: Vec<&'a Node>,
    left_ancestor_input_position_stack: Vec<Point>,
    left_ancestor_output_position_stack: Vec<Point>,

    right_ancestor: Option<&'a Node>,
    right_ancestor_input_position: Option<Point>,
    right_ancestor_output_position: Option<Point>,
    right_ancestor_stack: Vec<&'a Node>,
    right_ancestor_input_position_stack: Vec<Point>,
    right_ancestor_output_position_stack: Vec<Point>
}

impl<'a> Cursor<'a> {
    fn new(current_node: &'a mut Option<Rc<Node>>) -> Cursor {
        Cursor {
            current_node: current_node,
            input_start: Point::zero(),
            output_start: Point::zero(),

            left_ancestor: None,
            left_ancestor_input_position: None,
            left_ancestor_output_position: None,
            left_ancestor_stack: vec![],
            left_ancestor_input_position_stack: vec![],
            left_ancestor_output_position_stack: vec![],

            right_ancestor: None,
            right_ancestor_input_position: None,
            right_ancestor_output_position: None,
            right_ancestor_stack: vec![],
            right_ancestor_input_position_stack: vec![],
            right_ancestor_output_position_stack: vec![]
        }
    }
}
