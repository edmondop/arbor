use {
    std::{
        collections::VecDeque,
        fmt,
        sync::{Arc, Mutex},
        time::SystemTime,
    },
    tracing::{Event, Subscriber, field::Visit},
    tracing_subscriber::{Layer, layer::Context, registry::LookupSpan},
};

const MAX_LOG_ENTRIES: usize = 10_000;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub level: tracing::Level,
    pub target: String,
    pub message: String,
    pub fields: Vec<(String, String)>,
}

struct Inner {
    entries: VecDeque<LogEntry>,
    generation: u64,
}

#[derive(Clone)]
pub struct LogBuffer {
    inner: Arc<Mutex<Inner>>,
}

impl LogBuffer {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                entries: VecDeque::with_capacity(MAX_LOG_ENTRIES),
                generation: 0,
            })),
        }
    }

    pub fn generation(&self) -> u64 {
        self.inner.lock().map(|inner| inner.generation).unwrap_or(0)
    }

    pub fn snapshot(&self) -> Vec<LogEntry> {
        self.inner
            .lock()
            .map(|inner| inner.entries.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn push(&self, entry: LogEntry) {
        if let Ok(mut inner) = self.inner.lock() {
            if inner.entries.len() >= MAX_LOG_ENTRIES {
                inner.entries.pop_front();
            }
            inner.entries.push_back(entry);
            inner.generation += 1;
        }
    }
}

pub struct InMemoryLayer {
    buffer: LogBuffer,
}

impl InMemoryLayer {
    pub fn new(buffer: LogBuffer) -> Self {
        Self { buffer }
    }
}

impl<S> Layer<S> for InMemoryLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: *metadata.level(),
            target: metadata.target().to_owned(),
            message: visitor.message,
            fields: visitor.fields,
        };

        self.buffer.push(entry);
    }
}

#[derive(Default)]
struct FieldVisitor {
    message: String,
    fields: Vec<(String, String)>,
}

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{value:?}");
        } else {
            self.fields
                .push((field.name().to_owned(), format!("{value:?}")));
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_owned();
        } else {
            self.fields
                .push((field.name().to_owned(), value.to_owned()));
        }
    }
}
