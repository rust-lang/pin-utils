#![forbid(unsafe_code)] // pin_mut! is completely safe.

use core::pin::Pin;
use pin_utils::pin_mut;

#[test]
fn stack_pin() {
    struct Foo {}
    let foo = Foo {};
    pin_mut!(foo);
    let _: Pin<&mut Foo> = foo;

    let bar = Foo {};
    let baz = Foo {};
    pin_mut!(bar, baz,);
    let _: Pin<&mut Foo> = bar;
    let _: Pin<&mut Foo> = baz;
}
