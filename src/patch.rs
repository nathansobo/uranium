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

        let lower_bound = self.find_lower_bound(start);
        lower_bound.map(|node| self.splay_node(node.clone()));

        let upper_bound = self.find_upper_bound(&old_end);
        upper_bound.map(|node| self.splay_node(node.clone()));
    }

    fn find_lower_bound(&self, target: &Point) -> Option<Rc<Node<'a>>> {
        self.root.as_ref().and_then(|root| {
            let mut current_node: Rc<Node> = root.clone();
            let mut max_lower_bound: Option<Rc<Node>> = None;
            let mut left_ancestor_end = Point::zero();

            loop {
                let current_node_start = left_ancestor_end.traverse(&current_node.new_distance_from_left_ancestor);
                if target < &current_node_start {
                    if let None = current_node.left_child {
                        return max_lower_bound;
                    } else {
                        current_node = current_node.left_child.as_ref().unwrap().clone();
                    }
                } else {
                    max_lower_bound = Some(current_node.clone());

                    if let None = current_node.right_child {
                        return max_lower_bound;
                    } else {
                        left_ancestor_end = current_node_start.traverse(&current_node.new_extent);
                        current_node = current_node.right_child.as_ref().unwrap().clone();
                    }
                }
            }
        })
    }

    fn find_upper_bound(&self, target: &Point) -> Option<Rc<Node<'a>>> {
        self.root.as_ref().and_then(|root| {
            let mut current_node: Rc<Node> = root.clone();
            let mut min_upper_bound: Option<Rc<Node>> = None;
            let mut left_ancestor_end = Point::zero();

            loop {
                let current_node_end = left_ancestor_end
                    .traverse(&current_node.new_distance_from_left_ancestor)
                    .traverse(&current_node.new_extent);
                if target <= &current_node_end {
                    min_upper_bound = Some(current_node.clone());

                    if let None = current_node.left_child {
                        return min_upper_bound;
                    } else {
                        current_node = current_node.left_child.as_ref().unwrap().clone();
                    }
                } else {
                    if let None = current_node.right_child {
                        return min_upper_bound;
                    } else {
                        left_ancestor_end = current_node_end.traverse(&current_node.new_extent);
                        current_node = current_node.right_child.as_ref().unwrap().clone();
                    }
                }
            }
        })
    }

    fn splay_node(&mut self, node: Rc<Node>) {
    }

    fn rotate_node_right(&mut self, pivot: Rc<Node>) {
    }

    fn rotate_node_left(&mut self, pivot: Rc<Node>) {
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

impl<'a> Node<'a> {
    pub fn is_left_child(&self) -> bool {
        self.get_parent().as_ref()
            .and_then(|parent| {
                parent.left_child.as_ref().map(|left_sibling| {
                    self == &**left_sibling
                })
            })
            .unwrap_or(false)
    }

    pub fn is_right_child(&self) -> bool {
        self.get_parent().as_ref()
            .and_then(|parent| {
                parent.right_child.as_ref().map(|right_sibling| {
                    self == &**right_sibling
                })
            })
            .unwrap_or(false)
    }

    pub fn get_parent(&self) -> Option<Rc<Node<'a>>> {
        self.parent.as_ref().and_then(|parent| { parent.upgrade() })
    }
}
