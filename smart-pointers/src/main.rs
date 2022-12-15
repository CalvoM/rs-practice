use std::{
    alloc::{dealloc, Layout},
    mem,
    ops::Deref,
    rc::Rc,
};

use smart_pointers::MyBox;

fn hello_world(name: &str) {
    println!("Hello, {}", name);
}
fn raw_ptrs() {
    // Rarely used
    let mut x = 1i32;
    let mut_xptr: &mut i32 = &mut x; //works
    let xptr: &i32 = &x;
    // let mut_xptr: &mut i32 = &mut x; //won't work
    let xraw: *const i32 = xptr as *const i32;

    let mut y = 2i32;
    let yptr: &mut i32 = &mut y;
    let yraw: *mut i32 = yptr as *mut i32;
    let yraw2: *mut i32 = yptr as *mut i32;
    unsafe {
        println!("value is {}", *yraw);
        println!("value is {}", *yraw2);
    }
}
fn smart_ptrs_box() {
    let x = 2i32;
    let y: Box<i32> = Box::new(x);
    println!("{}", *y);
    println!("{}", y);
}

fn smart_ptrs_rc() -> Rc<String> {
    let s1: Rc<String> = Rc::new(String::from("Hello"));
    let s2: Rc<String> = s1.clone();
    s2
}

struct CustomBox<T> {
    value: *mut T,
}
impl<T> CustomBox<T> {
    fn new(t: T) -> CustomBox<T> {
        let value = unsafe {
            let typesize = mem::size_of::<T>();
            let layout = Layout::from_size_align(typesize, typesize).unwrap();
            println!("Layout size: {}", layout.size());
            println!("Layout align: {}", layout.align());
            let mut value = std::alloc::alloc(layout) as *mut T;
            std::ptr::copy(&t as *const T, value, typesize);
            value
        };
        mem::forget(t);
        CustomBox { value }
    }
}
impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.value }
    }
}
impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {
        unsafe {
            let typesize = mem::size_of::<T>();
            let layout = Layout::from_size_align(typesize, typesize).unwrap();
            dealloc(self.value as *mut u8, layout);
        }
    }
}

struct CustomRc<T> {
    count: *mut i32,
    value: *mut T,
}

impl<T> CustomRc<T> {
    fn new(t: T) -> CustomRc<T> {
        CustomRc {
            count: Box::into_raw(Box::new(1)),
            value: Box::into_raw(Box::new(t)),
        }
    }
}
fn main() {
    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // // y.drop(); cannot be called manually
    // drop(y);
    // let name = MyBox::new(String::from("Kazi ya Msalaba"));
    // hello_world(&name);
    raw_ptrs();
    smart_ptrs_box();
    let boxy: CustomBox<u32> = CustomBox::new(6);
    let value = boxy.deref();
    let value2 = boxy.deref();
    println!("{}", boxy.deref());
    println!("{} {}", *boxy, *(boxy.deref()));
}
