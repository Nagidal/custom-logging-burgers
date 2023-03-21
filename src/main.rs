#[allow(unused)]
use tracing::{debug_span, info, info_span};
use tracing_subscriber::layer::SubscriberExt as _; // adds .with()
use tracing_subscriber::util::SubscriberInitExt as _; // adds .init()

mod custom_layer;
use custom_layer::CustomLayer;

fn main() {
    // Set up how `tracing-subscriber` will deal with tracing data
    tracing_subscriber::registry().with(CustomLayer).init();

    let outer_span = info_span!("outer", datum = 0);
    let _outer_entered = outer_span.enter();
    info!("hi from outer");

    let inner_span = debug_span!("inner", ipsum = 1);
    let _inner_span = inner_span.enter();
    info!("hi from inner");

    // Log something simple, In `tracing` parlance, this creates an "event"
    //info!(a_bool = true, answer = 42, message = "first example");
    //info!("hi there");
}
