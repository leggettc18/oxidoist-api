#[macro_use]
extern crate derive_builder;

pub mod api;
pub mod project;
pub mod section;
pub mod task;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
