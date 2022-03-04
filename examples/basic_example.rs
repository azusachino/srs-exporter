use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};

fn main() {
    let counter_opts = Opts::new("test_counter", "test_counter_help");
    let counter = Counter::with_opts(counter_opts).unwrap();

    let r = Registry::new();
    r.register(Box::new(counter.clone())).unwrap();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = r.gather();

    encoder.encode(&metric_families, &mut buffer).unwrap();

    println!("{}", String::from_utf8(buffer).unwrap());
}
