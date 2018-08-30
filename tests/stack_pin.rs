#![feature(pin)]
use pin_utils::pin_mut;
use core::pin::PinMut;

#[test]
fn stack_pin() {
    struct Foo {}
    let foo = Foo {};
    pin_mut!(foo);
    let _: PinMut<Foo> = foo;
}
