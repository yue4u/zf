use gdnative::prelude::*;
use tracing::{field::Visit, Level};
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer};

struct GodotProxyLayerVistitor(Option<String>);

impl Visit for GodotProxyLayerVistitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0 = Some(format!("{:?}", value))
        }
    }
}

struct GodotProxyLayer;

impl<S> Layer<S> for GodotProxyLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let metadata = event.metadata();
        let mut msg_vistor = GodotProxyLayerVistitor(None);
        event.record(&mut msg_vistor);
        let msg = format!(
            "[{}] {} (at {})",
            metadata.level(),
            msg_vistor.0.unwrap_or_default(),
            metadata.name()
        );
        match *metadata.level() {
            Level::TRACE | Level::DEBUG | Level::INFO => {
                godot_print!("{}", msg);
            }
            Level::WARN => {
                godot_warn!("{}", msg);
            }
            Level::ERROR => {
                godot_error!("{}", msg);
            }
        };
    }
}

pub fn init() {
    let filter = filter::Targets::new().with_target("zf", Level::TRACE);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(GodotProxyLayer)
        .with(filter)
        .init();
}
