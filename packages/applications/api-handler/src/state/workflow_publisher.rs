use aws_config::SdkConfig;
use workflow_dispatcher_protocol::DispatchWorkflow;

#[derive(Debug, Clone)]
pub struct WorkflowPublisher {
    queue_url: String,
    client: aws_sdk_sqs::Client,
}

impl WorkflowPublisher {
    pub fn new(queue_url: &str, sdk_config: &SdkConfig) -> Self {
        Self {
            queue_url: queue_url.to_string(),
            client: aws_sdk_sqs::Client::new(sdk_config),
        }
    }

    pub async fn dispatch_workflow(
        &self,
        message: DispatchWorkflow,
    ) -> anyhow::Result<()> {
        self.client
            .send_message()
            .queue_url(&self.queue_url)
            .message_body(serde_json::to_string(&message)?)
            .send()
            .await?;

        Ok(())
    }
}
