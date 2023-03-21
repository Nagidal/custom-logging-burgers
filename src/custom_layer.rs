use serde_json;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber};
use tracing_core::span::{Attributes, Id, Record};
use tracing_subscriber::layer::Context;
use tracing_subscriber::{registry::LookupSpan, Layer};

pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer
where
    // see https://docs.rs/tracing-subscriber/0.3.16/tracing_subscriber/layer/struct.Context.html
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        // Build our json object from the field values like we have been
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        attrs.record(&mut visitor);

        // And move it in our storage type
        let storage = CustomFieldStorage(fields);

        // Get a reference to the internal span data
        let span = ctx.span(id).expect("did not find span, this is a bug");
        // Get the special place where tracing-subscriber stores custom data
        let mut extensions = span.extensions_mut();
        extensions.insert(storage);
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        // Get the span whose data is being recorded
        let span = ctx.span(span).expect("Did not get span, bug");
        // Get a mutable reference to the data we created in on_new_span
        let mut extensions_mut = span.extensions_mut();
        let cfs: &mut CustomFieldStorage = extensions_mut
            .get_mut::<CustomFieldStorage>()
            .expect("Did not get mutable extensions, bug");
        let json_data: &mut BTreeMap<String, serde_json::Value> = &mut cfs.0;
        // And add to it using our old friend the JsonVisitor
        let mut visitor = JsonVisitor(json_data);
        values.record(&mut visitor);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Extract the span data from each span and its extensions and store it
        // temporarily in a vector
        let mut span_data: Vec<serde_json::Value> = vec![];
        match ctx.event_scope(event) {
            Some(spans) => {
                for span in spans.from_root() {
                    let extensions = span.extensions();
                    let storage = extensions
                        .get::<CustomFieldStorage>()
                        .expect("Did not get extensions");
                    let field_data: &BTreeMap<String, serde_json::Value> = &storage.0;
                    span_data.push(serde_json::json!({
                        "target": span.metadata().target(),
                        "name": span.name(),
                        "level": format!("{}", span.metadata().level()),
                        "fields": field_data,
                    }));
                }
            }
            None => {}
        };

        // Get the fields of the Event
        let mut event_fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut event_fields);
        event.record(&mut visitor);
        let output = serde_json::json!({
            "target": event.metadata().target(),
            "name": event.metadata().name(),
            "level": format!("{}", event.metadata().level()),
            "fields": event_fields,
            "spans": span_data,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&output).expect("No output created!")
        );
    }
}

#[derive(Debug)]
struct CustomFieldStorage(BTreeMap<String, serde_json::Value>);

struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<'a> Visit for JsonVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }
    fn record_i128(&mut self, field: &Field, value: i128) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }
    fn record_u128(&mut self, field: &Field, value: u128) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }
    fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{}", value)),
        );
    }
}

struct PrintlnVisitor;

#[allow(unused)]
impl PrintlnVisitor {
    fn new() -> Self {
        Self {}
    }
}

impl Visit for PrintlnVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        println!("    field={field} value={value:?}")
    }
    fn record_f64(&mut self, field: &Field, value: f64) {
        println!("    field={field} value={value}")
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        println!("    field={field} value={value}")
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        println!("    field={field} value={value}")
    }
    fn record_i128(&mut self, field: &Field, value: i128) {
        println!("    field={field} value={value}")
    }
    fn record_u128(&mut self, field: &Field, value: u128) {
        println!("    field={field} value={value}")
    }
    fn record_bool(&mut self, field: &Field, value: bool) {
        println!("    field={field} value={value}")
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        println!("    field={field} value={value}")
    }
    fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)) {
        println!("    field={field} value={value}")
    }
}
