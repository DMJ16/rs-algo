mod linked_list;

use linked_list::{doubly, singly};

fn main() {
    let mut list = doubly::LinkedList::new();

    list.append(String::from("David"));
    list.append(String::from("David"));
    list.append(String::from("David"));
    list.pop();
    println!("{:?}", list);
}
