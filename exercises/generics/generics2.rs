// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



// struct Wrapper {
//     value: Box<dyn Any>,
// }

// impl Wrapper {
//     pub fn new(value: impl Any) -> Self {
//         Wrapper { value:Box::new(value) }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn store_u32_in_wrapper() {
//         let wrapper = Wrapper::new(42u32);
//         assert_eq!(wrapper.value.downcast_ref::<u32>().unwrap(), &42);
//     }

//     #[test]
//     fn store_str_in_wrapper() {
//         let wrapper = Wrapper::new("Foo");
//         assert_eq!(wrapper.value.downcast_ref::<u32>().unwrap(),,&"Foo");
//     }
// }
