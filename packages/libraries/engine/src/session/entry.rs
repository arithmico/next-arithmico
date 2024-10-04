use super::error::SessionError;

pub struct SessionEntry {
    pub input: String,
    pub output: Result<String, SessionError>,
}
