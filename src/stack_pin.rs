/// Pins a value on the stack.
///
/// ```
/// # #![feature(pin)]
/// # #[macro_use] extern crate pin_utils;
/// # use core::mem::PinMut;
/// # struct Foo {}
/// let foo = Foo { /* ... */ };
/// pin_mut!(foo);
/// let _: PinMut<Foo> = foo;
/// ```
#[macro_export]
macro_rules! pin_mut {
    ($($x:ident),*) => { $(
        // Move the value to ensure that it is owned
        let mut $x = $x;
        // Shadow the original binding so that it can't be directly accessed
        // ever again.
        #[allow(unused_mut)]
        let mut $x = unsafe {
            $crate::core_reexport::mem::PinMut::new_unchecked(&mut $x)
        };
    )* }
}
