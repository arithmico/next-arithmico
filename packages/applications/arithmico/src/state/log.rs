use common::{AngleUnit, Language};
use engine::SessionError;
use node::Node;

#[derive(Debug, Clone, Default)]
pub struct Log {
    pub entries: Vec<LogEntry>,
}

impl Log {
    pub fn push(
        &mut self,
        language: Language,
        angle_unit: AngleUnit,
        input: String,
    ) {
        self.entries
            .push(LogEntry::new(language, angle_unit, input));
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub language: Language,
    pub angle_unit: AngleUnit,
    pub input: String,
    pub output: Option<Result<Node, SessionError>>,
}

impl LogEntry {
    pub fn new(
        language: Language,
        angle_unit: AngleUnit,
        input: String,
    ) -> Self {
        Self {
            language,
            angle_unit,
            input,
            output: None,
        }
    }
}
