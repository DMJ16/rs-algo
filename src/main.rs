mod linked_list;

use linked_list::List;

fn main() {
    let mut list = List::new();

    list = list.prepend(10);
    list = list.prepend(10);
    list = list.prepend(10);

    println!("linked list has length of {}", list.len());
    println!("{}", list.stringify());
}
