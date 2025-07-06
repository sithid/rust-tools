mod lists;

use lists::LinkedList;
use lists::Node;

use rand::Rng;
fn main() {
  println!("\n\r******Linked List Testing Output******\n\r");
  let mut list = LinkedList::new();

  let mut rng = rand::thread_rng();

  for _i in 0..10 {
    let val: i64 = rng.gen_range(1..=1000000);
    println!("Pushing {} to the list", val);
    list.push(val);
  }

  println!("\n\r");
  println!("Finished building linked list: Size = {}", list.size);
  println!("\n\r");

  let mut next: &Option<Box<Node>> = &list.head;
  let mut index: i32 = 0;

  while let Some(node) = next {
    println!("Node Position: {}, Value: {}", index, node.value);
    next = &node.next;
    index += 1;
  }

  println!("\n\rLinked List Size: {}\n\r", list.size);

  while list.is_empty() {
    let val: i64 = list.pop().take().expect("value should not be empty");
    println!("Value Popped: {}, List Size: {}", val, list.size);

    if !list.is_empty() {
      let head_value = list.peek().take().expect("value should not be empty");
      println!("Linked List Peek Head: {}\n\r", head_value);
    }
  }
}