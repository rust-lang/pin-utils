#![feature(pin)]
#[macro_use] extern crate pin_utils;
use core::pin::PinMut;

#[test]
fn stack_pin() {
    struct Foo {}
    let foo = Foo {};
    pin_mut!(foo);
    let _: PinMut<Foo> = foo;
}
