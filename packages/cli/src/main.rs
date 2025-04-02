use extractor::base::KST;
fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();
    let offset = KST.to_offset();
    tracing::debug!("offset: {}", offset);
}
