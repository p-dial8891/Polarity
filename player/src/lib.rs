use std::io::Error;
use opentelemetry::trace::TracerProvider as _;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

#[tarpc::service]
pub trait Player {
    async fn play(path: String) -> Result<(),()>;
	async fn skip() -> Result<(),()>;
}

/// Initializes an OpenTelemetry tracing subscriber with a OTLP backend.
pub fn init_tracing(service_name: &'static str) -> anyhow::Result<()> {
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            opentelemetry_sdk::Resource::new([opentelemetry::KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                service_name,
            )]),
        ))
        .with_batch_config(opentelemetry_sdk::trace::BatchConfig::default())
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;
    opentelemetry::global::set_tracer_provider(tracer_provider.clone());
    let tracer = tracer_provider.tracer(service_name);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    Ok(())
}
