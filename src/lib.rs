macro_rules! bug_if_async_trait {
    ($serde_path:literal) => {
        #[derive(serde::Deserialize)]
        #[serde(crate = $serde_path)]
        struct Foo;
    };
}

bug_if_async_trait! {
    "serde"
}
