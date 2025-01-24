pub mod easy;
pub mod medium;
pub mod solution;
pub mod traits;

pub use traits::Solver; // 重新导出 Solver trait

#[macro_export]
macro_rules! impl_with_id {
    ($($type:ty),*) => {
        $(
            impl $type {
                pub fn with_id(id: usize) -> Self {
                    Self { id }
                }
            }
        )*
    };
}