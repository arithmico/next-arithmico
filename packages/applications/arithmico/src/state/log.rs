use common::{AngleUnit, Language};
use engine::SessionError;
use leptos::logging::warn;
use node::Node;

#[derive(Debug, Clone, Default)]
pub struct Log {
    pub entries: Vec<LogEntry>,
}

impl Log {
    pub fn push(
        &mut self,
        correlation_id: usize,
        language: Language,
        angle_unit: AngleUnit,
        input: String,
    ) {
        self.entries.push(LogEntry::new(
            correlation_id,
            language,
            angle_unit,
            input,
        ));
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn set_output(
        &mut self,
        correlation_id: usize,
        output: Result<Node, SessionError>,
    ) {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.correlation_id == correlation_id)
        {
            if entry.output.is_some() {
                warn!(
                    "Log entry output for correlation_id {} was already updated!",
                    correlation_id
                );
            } else {
                entry.output = Some(output);
            }
        } else {
            warn!("Unknown correlation_id {}", correlation_id);
        }
    }

    pub fn last(&self) -> Option<&LogEntry> {
        self.entries.last()
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub correlation_id: usize,
    #[allow(dead_code)]
    pub language: Language,
    #[allow(dead_code)]
    pub angle_unit: AngleUnit,
    pub input: String,
    pub output: Option<Result<Node, SessionError>>,
}

impl LogEntry {
    pub fn new(
        correlation_id: usize,
        language: Language,
        angle_unit: AngleUnit,
        input: String,
    ) -> Self {
        Self {
            correlation_id,
            language,
            angle_unit,
            input,
            output: None,
        }
    }
}
