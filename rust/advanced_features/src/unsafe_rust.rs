pub fn raw_pointer()
{
    let mut num = 5;

    // use as to cast immutable and mutable
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let _r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("r is: {}", *r); // SIGSEGV
    }

    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

// The following function cannot compile since we cannot borrow one thing as mutable more than once at a time
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//
//     assert!(mid <= len);
//
//     (&mut slice[..mid], &mut slice[mid..]) // cannot borrow `*slice` as mutable more than once at a time
// }

unsafe fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    let mid_r = &mut slice[mid..] as *mut [i32]; // not a rust-style

    (&mut slice[..mid], mid_r.as_mut().unwrap())
}

use std::slice;

fn split_at_mut2(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn test_split() {
    let mut a = [1, 2, 3, 4, 5];

    unsafe {
        let (a_1, a_2) = split_at_mut(&mut a, 3);
        println!("{:?}, {:?}", a_1, a_2);
        a_1[0] = 3;
        let (a_1, a_2) = split_at_mut2(&mut a, 3);
        println!("{:?}, {:?}", a_1, a_2);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn test_c_abs() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

static HELLO_WORLD: &str = "Hello, world!"; // global reference can only be 'static and could be omitted

pub fn test_global_var() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe  {
        COUNTER += inc;
    }
}
