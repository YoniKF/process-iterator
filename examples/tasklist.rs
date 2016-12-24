extern crate process_iterator;

use process_iterator::ProcessIterator;

fn main() {
    for entry in ProcessIterator::new().unwrap() {
        println!("{:?}", entry);
    }
}