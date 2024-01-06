pub mod paths;
pub mod models;
pub mod resources;
pub mod data_loader;

mod util;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
