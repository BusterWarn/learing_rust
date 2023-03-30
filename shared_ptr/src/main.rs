use std::{rc::Rc, cell::RefCell};

fn main() {
  let mut my_vec : Vec<Rc<RefCell<u32>>> = Vec::with_capacity(10);

  for i in 0..3 as u32
  {
    my_vec.push(Rc::new(RefCell::new(i)));
    println!("i: {} address {:p}", i, my_vec.last().unwrap());
  }

  {
    let rc = my_vec.get(2).unwrap();
    let new_clone = Rc::clone(rc);
    println!("\nTotal reference count? {}", Rc::strong_count(&new_clone));

    let mut mutable_value = new_clone.borrow_mut();
    println!("u: {} address: {:p}", mutable_value, new_clone);

    *mutable_value += 100;   
  }

  let rc = my_vec.get(2).unwrap();
  println!("\nTotal reference count? {}\n", Rc::strong_count(&rc));

  for rc in my_vec
  {
    println!("rc_value: {} address {:p}", rc.borrow_mut(), rc);
  }
}
