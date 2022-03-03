mod resource_usage;
use anyhow::Result;
pub use resource_usage::ResourceUsage;

mod stream_usage;
pub use stream_usage::StreamUsage;

pub trait Collector {
    fn collect(&self) -> Result<String>;
}
