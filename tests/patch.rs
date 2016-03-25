extern crate uranium;

use uranium::point::Point;
use uranium::patch::*;

#[cfg(test)]
fn non_overlapping_splices() {
    let patch = Patch::new();

    patch.splice_with_text(&Point::new(0, 3), &Point::new(0, 4), "hello");
    patch.splice_with_text(&Point::new(0, 10), &Point::new(0, 5), "hello");
}
