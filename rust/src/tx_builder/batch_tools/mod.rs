cfg_if! {
    if #[cfg(test)] {
        mod testing;
    } else if #[cfg(feature = "property-test-api")] {
        pub mod testing;
    }
}

pub mod abstract_map;