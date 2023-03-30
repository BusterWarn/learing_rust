# shared_ptr in Rust

Trying to see how you can get something similar to a C++ [shared_ptr](https://en.cppreference.com/w/cpp/memory/shared_ptr). [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) handles the reference counting while [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) handles... Mutability.

Very weird, and I don't fully get how the addresses work. But here is the output:

```bash
i: 0 address 0x24aa96e4610
i: 1 address 0x24aa96e4618
i: 2 address 0x24aa96e4620

Total reference count? 2
u: 2 address: 0x24aa96efce0

Total reference count? 1

rc_value: 0 address 0x24aa96efc50
rc_value: 1 address 0x24aa96efc20
rc_value: 102 address 0x24aa96efce0
```