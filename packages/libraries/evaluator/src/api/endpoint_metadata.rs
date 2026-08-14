use translate_core::RenderedTranslatedMessage;

#[derive(Debug)]
pub struct EndpointMetadata {
    pub(crate) endpoint_name: String,
    pub(crate) module_id: String,
    pub(crate) module_name: RenderedTranslatedMessage,
    pub(crate) description: RenderedTranslatedMessage,
}

impl EndpointMetadata {
    pub fn new(
        endpoint_name: String,
        module_id: String,
        module_name: RenderedTranslatedMessage,
        description: RenderedTranslatedMessage,
    ) -> Self {
        EndpointMetadata {
            endpoint_name,
            module_id,
            module_name,
            description,
        }
    }

    pub fn endpoint_name(&self) -> &str {
        &self.endpoint_name
    }

    pub fn module_id(&self) -> &str {
        &self.module_id
    }

    pub fn module_name(&self) -> &RenderedTranslatedMessage {
        &self.module_name
    }

    pub fn description(&self) -> &RenderedTranslatedMessage {
        &self.description
    }
}
