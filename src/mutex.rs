#[cfg(feature = "mutex-futures")]
pub const NAME: &str = "futures::lock::Mutex";
#[cfg(feature = "mutex-futures")]
pub type Mutex<T> = futures::lock::Mutex<T>;

#[cfg(feature = "mutex-async-std")]
pub const NAME: &str = "async_std::sync::Mutex";

#[cfg(feature = "mutex-async-std")]
pub type Mutex<T> = async_std::sync::Mutex<T>;

#[cfg(feature = "mutex-tokio")]
pub const NAME: &str = "tokio::sync::Mutex";

#[cfg(feature = "mutex-tokio")]
pub type Mutex<T> = tokio::sync::Mutex<T>;

#[cfg(all(feature = "mutex-futures-intrusive", feature = "mutex-futures-intrusive-fair"))]
pub const NAME: &str = "futures_intrusive::sync::Mutex(fair = true)";
#[cfg(all(feature = "mutex-futures-intrusive", not(feature = "mutex-futures-intrusive-fair")))]
pub const NAME: &str = "futures_intrusive::sync::Mutex(fair = false)";

#[cfg(feature = "mutex-futures-intrusive")]
pub type Mutex<T> = futures_intrusive::sync::Mutex<T>;

#[cfg(all(feature = "mutex-futures-intrusive", feature = "mutex-futures-intrusive-fair"))]
pub const IS_FAIR: bool = true;

#[cfg(all(feature = "mutex-futures-intrusive", not(feature = "mutex-futures-intrusive-fair")))]
pub const IS_FAIR: bool = false;