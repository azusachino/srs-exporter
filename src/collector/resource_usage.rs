const BASE_URL: &str = "/api/v1/rusage/";

pub struct ResourceUsage {}

impl ResourceUsage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(self) {
        println!("{}", BASE_URL);
    }
}
