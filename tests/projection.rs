#![feature(pin, arbitrary_self_types)]
#[macro_use] extern crate pin_utils;
use std::marker::Unpin;
use std::pin::PinMut;

struct Foo<T1, T2> {
    field1: T1,
    field2: T2,
}

impl<T1, T2> Foo<T1, T2> {
    unsafe_pinned!(field1: T1);
    unsafe_unpinned!(field2: T2);
}

impl<T1: Unpin, T2> Unpin for Foo<T1, T2> {} // Conditional Unpin impl

#[test]
fn projection() {
    let foo = Foo { field1: 1, field2: 2 };
    pin_mut!(foo);

    let x1: PinMut<i32> = foo.field1();
    assert_eq!(*x1, 1);

    let x2: &mut i32 = foo.field2();
    assert_eq!(*x2, 2);
}
