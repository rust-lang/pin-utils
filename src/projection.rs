/// A pinned projection of a struct field.
///
/// To make using this macro safe, three things need to be ensured:
/// - If the struct implements [`Drop`], the [`drop`] method is not allowed to
///   move the value of the field.
/// - If the struct wants to implement [`Unpin`], it has to do so conditionally:
///   The struct can only implement [`Unpin`] if the field's type is [`Unpin`].
/// - The struct must not be `#[repr(packed)]`.
///
/// ```
/// # #![feature(arbitrary_self_types)]
/// # use pin_utils::unsafe_pinned;
/// # use std::pin::Pin;
/// # use std::marker::Unpin;
/// struct Foo<T> {
///     field: T,
/// }
///
/// impl<T> Foo<T> {
///     unsafe_pinned!(field: T);
///
///     fn baz(mut self: Pin<&mut Self>) {
///         let _: Pin<&mut T> = self.field(); // Pinned reference to the field
///     }
/// }
///
/// impl<T: Unpin> Unpin for Foo<T> {} // Conditional Unpin impl
/// ```
///
/// [`Unpin`]: core::marker::Unpin
/// [`drop`]: Drop::drop
#[macro_export]
macro_rules! unsafe_pinned {
    ($f:tt: $t:ty) => (
        fn $f<'__a>(
            self: &'__a mut $crate::core_reexport::pin::Pin<&mut Self>
        ) -> $crate::core_reexport::pin::Pin<&'__a mut $t> {
            unsafe {
                $crate::core_reexport::pin::Pin::map_unchecked_mut(
                    self.as_mut(), |x| &mut x.$f
                )
            }
        }
    )
}

/// An unpinned projection of a struct field.
///
/// This macro is unsafe because it creates a method that returns a normal
/// non-pin reference to the struct field. It is up to the programmer to ensure
/// that the contained value can be considered not pinned in the current
/// context.
///
/// ```
/// # #![feature(arbitrary_self_types)]
/// # use pin_utils::unsafe_unpinned;
/// # use std::pin::Pin;
/// # struct Bar;
/// struct Foo {
///     field: Bar,
/// }
///
/// impl Foo {
///     unsafe_unpinned!(field: Bar);
///
///     fn baz(mut self: Pin<&mut Self>) {
///         let _: &mut Bar = self.field(); // Normal reference to the field
///     }
/// }
/// ```
#[macro_export]
macro_rules! unsafe_unpinned {
    ($f:tt: $t:ty) => (
        fn $f<'__a>(
            self: &'__a mut $crate::core_reexport::pin::Pin<&mut Self>
        ) -> &'__a mut $t {
            unsafe {
                &mut $crate::core_reexport::pin::Pin::get_unchecked_mut(
                    self.as_mut()
                ).$f
            }
        }
    )
}
