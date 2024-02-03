pub mod paths {
    use ignore::WalkBuilder;

    pub fn list() {
        for result in WalkBuilder::new("./").build() {
            println!("{:?}", result);
        }
    }
}
