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
