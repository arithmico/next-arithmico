use super::error::SessionError;

#[derive(Debug, Clone)]
pub struct SessionEntry {
    pub input: String,
    pub output: Result<String, SessionError>,
}
