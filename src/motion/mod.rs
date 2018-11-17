pub mod animated;
pub mod composition;

pub use self::animated::{Animated, Constant};
pub use self::composition::Composition;
pub use self::composition::compose;