use env_logger::{Builder, Target};

fn main() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);

    builder.init();

    let cfg = deduplicator::cli::parse_config();

    deduplicator::scan(cfg);
}
