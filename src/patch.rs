use std::rc::Rc;
use std::rc::Weak;
use point::Point;

pub struct Patch<'a> {
    root: Option<Rc<Node<'a>>>,
}

impl<'a> Patch<'a> {
    pub fn new() -> Patch<'a> {
        Patch {root: None}
    }

    pub fn splice_with_text(&'a mut self, start: &Point, old_text: &str, new_text: &str) {
        let old_extent = Point::extent(old_text);
        let new_extent = Point::extent(new_text);
        self.splice(start, &old_extent, &new_extent, old_text, new_text);
    }

    pub fn splice(&'a mut self, start: &Point, old_extent: &Point, new_extent: &Point, old_text: &str, new_text: &str) {
        if old_extent.is_zero() && new_extent.is_zero() { return; }

        let output_old_end = start.traverse(old_extent);
        let output_new_end = start.traverse(new_extent);

        let cursor = Cursor::new(&mut self.root);
        let start_node = cursor.insert_splice_boundary(start, None);
        start_node.is_change_start = true;
        self.splay_node(start_node);

        let end_node = cursor.insert_splice_boundary(&output_old_end, Some(start_node));
        end_node.is_change_end = true;
        self.splay_node(end_node);
        if end_node.left_child.is_some() && start_node == end_node.left_child.unwrap() {
            self.rotate_node_right(start_node)
        }
    }

    fn splay_node(&'a mut self, node: Rc<Node>) {

    }

    fn rotate_node_right(&'a mut self, pivot: Rc<Node>) {
    }

    fn rotate_node_left(&'a mut self, pivot: Rc<Node<'a>>) {
    }
}

pub struct Node<'a> {
    id: u32,
    parent: Option<Weak<Node<'a>>>,
    left_child: Option<Rc<Node<'a>>>,
    right_child: Option<Rc<Node<'a>>>,
    is_change_start: bool,
    is_change_end: bool,
    input_left_extent: Point,
    output_left_extent: Point,
    input_extent: Point,
    output_extent: Point,
    new_text: &'a str,
    old_text: &'a str
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}

struct Cursor<'a> {
    current_node: &'a mut Option<Rc<Node<'a>>>,
    input_start: Point,
    output_start: Point,

    left_ancestor: Option<&'a Node<'a>>,
    left_ancestor_input_position: Option<Point>,
    left_ancestor_output_position: Option<Point>,
    left_ancestor_stack: Vec<&'a Node<'a>>,
    left_ancestor_input_position_stack: Vec<Point>,
    left_ancestor_output_position_stack: Vec<Point>,

    right_ancestor: Option<&'a Node<'a>>,
    right_ancestor_input_position: Option<Point>,
    right_ancestor_output_position: Option<Point>,
    right_ancestor_stack: Vec<&'a Node<'a>>,
    right_ancestor_input_position_stack: Vec<Point>,
    right_ancestor_output_position_stack: Vec<Point>
}

impl<'a> Cursor<'a> {
    fn new(current_node: &'a mut Option<Rc<Node<'a>>>) -> Cursor<'a> {
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

    fn insert_splice_boundary(&self, point: &Point, start_node: Option<Rc<Node<'a>>>) -> Rc<Node<'a>> {
    }
}
