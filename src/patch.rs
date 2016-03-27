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

        let old_end = start.traverse(old_extent);
        let new_end = start.traverse(new_extent);

        let lower_bound = self.find_lower_bound(start).map(|node| self.splay_node(node));
        let upper_bound = self.find_upper_bound(&old_end).map(|node| self.splay_node(node));

    }

    fn find_lower_bound(&self, target: &Point) -> Option<Rc<Node>> {
        self.root.and_then(|root| {
            let mut left_ancestor_end = Point::zero();
            let mut max_lower_bound: Option<Rc<Node>> = None;
            let mut current_node = root;

            loop {
                let current_node_start = left_ancestor_end.traverse(&current_node.new_distance_from_left_ancestor);
                if target < &current_node_start {
                    match current_node.left_child {
                        Some(left_child) => {
                            current_node = left_child;
                        },
                        None => {
                            return max_lower_bound;
                        }
                    }
                } else {
                    max_lower_bound = Some(current_node);
                    match current_node.right_child {
                        Some(right_child) => {
                            current_node = right_child;
                            left_ancestor_end = current_node_start.traverse(&current_node.new_extent);
                        },
                        None => {
                            return max_lower_bound;
                        }
                    }
                }
            }
        })
    }

    fn find_upper_bound(&self, target: &Point) -> Option<Rc<Node>> {
        self.root.and_then(|root| {
            let mut left_ancestor_end = Point::zero();
            let mut min_upper_bound: Option<Rc<Node>> = None;
            let mut current_node = root;

            loop {
                let current_node_end = left_ancestor_end
                    .traverse(&current_node.new_distance_from_left_ancestor)
                    .traverse(&current_node.new_extent);
                if target < &current_node_end {
                    min_upper_bound = Some(current_node);
                    match current_node.left_child {
                        Some(left_child) => {
                            current_node = left_child;
                        },
                        None => {
                            return min_upper_bound;
                        }
                    }
                } else {
                    match current_node.right_child {
                        Some(right_child) => {
                            current_node = right_child;
                            left_ancestor_end = current_node_end
                        },
                        None => {
                            return min_upper_bound;
                        }
                    }
                }
            }
        })
    }

    fn splay_node(&'a mut self, node: Rc<Node>) -> Rc<Node> {

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
    old_distance_from_left_ancestor: Point,
    new_distance_from_left_ancestor: Point,
    old_extent: Point,
    new_extent: Point,
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
}
