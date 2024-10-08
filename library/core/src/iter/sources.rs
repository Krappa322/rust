mod empty;
mod from_coroutine;
mod from_fn;
mod once;
mod once_with;
mod repeat;
mod repeat_n;
mod repeat_with;
mod successors;

#[stable(feature = "iter_empty", since = "1.2.0")]
pub use self::empty::{empty, Empty};
#[unstable(
    feature = "iter_from_coroutine",
    issue = "43122",
    reason = "coroutines are unstable"
)]
pub use self::from_coroutine::from_coroutine;
#[stable(feature = "iter_from_fn", since = "1.34.0")]
pub use self::from_fn::{from_fn, FromFn};
#[stable(feature = "iter_once", since = "1.2.0")]
pub use self::once::{once, Once};
#[stable(feature = "iter_once_with", since = "1.43.0")]
pub use self::once_with::{once_with, OnceWith};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::repeat::{repeat, Repeat};
#[stable(feature = "iter_repeat_n", since = "CURRENT_RUSTC_VERSION")]
pub use self::repeat_n::{repeat_n, RepeatN};
#[stable(feature = "iterator_repeat_with", since = "1.28.0")]
pub use self::repeat_with::{repeat_with, RepeatWith};
#[stable(feature = "iter_successors", since = "1.34.0")]
pub use self::successors::{successors, Successors};
