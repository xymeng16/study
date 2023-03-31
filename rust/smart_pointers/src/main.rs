//! This project talks about smart pointers in Rust.
//! Smart pointers, are data structures that not only
//! act like a pointer but also have additional metadata
//! and capabilities.
//!
//! In Rust, which uses the concept of ownership and borrowing,
//! an additional difference between references and smart
//! pointers is that references are pointers that only borrow
//! data; in contrast, in many cases, smart pointers own the data they point to.
//! Some common smart pointers:
//! - `Box<T>` for allocating values on the heap
//! - `Rc<T>`, a reference counting type that enables multiple ownership
//! - `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type
//! that enforces the borrowing rules at runtime instead of compile time

fn main() {
    Box::store_on_heap();
    Box::con_list();
    Box::deref_example();
    Box::deref_box();
    Box::deref_mybox();
    Box::deref_coercion_mybox();

    Drop::drop_mysp();

    Rc::list_rc();
    Rc::count_rc();

    RefCell::refcell_in_rc();

    ref_cycle::ref_cycle();

    ref_cycle::make_tree();
    ref_cycle::make_tree2();
}
