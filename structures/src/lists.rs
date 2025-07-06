pub struct Node {
  pub value: i64,
  pub next: Option<Box<Node>>
}

impl Node {
  pub fn new( value: i64 ) -> Node {
    Node {
      value,
      next: None
    }
  }
}

pub struct LinkedList {
  pub head: Option<Box<Node>>, // rust's way of having null starting values for structs
  pub size: u32, // unsigned positive number 
}

impl LinkedList {
  pub fn new() -> LinkedList {
    LinkedList {
      head: None,
      size: 0
    }
  }

  pub fn push(&mut self, value: i64 ) {
    let new_node = Node {
      value,
      next: self.head.take()
    };

    self.head = Some(Box::new(new_node));
    self.size += 1;
  }

  pub fn pop(&mut self ) -> Option<i64> {
    self.head.take().map(|old_head: Box<Node>| {
      self.head = old_head.next;
      self.size -= 1;
      old_head.value
    })
  }

  pub fn peek(&self) -> Option<i64> {
    self.head.as_ref().map(|node| &node.value ).copied()
  }

  pub fn is_empty(&self) -> bool {
    self.head.is_none()
  }
}