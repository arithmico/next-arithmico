import { Duration } from "aws-cdk-lib";
import { Queue } from "aws-cdk-lib/aws-sqs";
import { Construct } from "constructs";

export interface WorkflowQueueProps {
    environment: string;
}

export class WorkflowQueue extends Construct {
    public readonly queue: Queue;
    public readonly deadLetterQueue: Queue;

    constructor(scope: Construct, id: string, props: WorkflowQueueProps) {
        super(scope, id)

        this.deadLetterQueue = new Queue(this, "DeadLetterQueue", {
            queueName: `${props.environment}.config-service.workflows.dispatch.v1.dlq`,
            enforceSSL: true,
            retentionPeriod: Duration.days(14),
        });

        this.deadLetterQueue
            .metricApproximateNumberOfMessagesVisible()
            .createAlarm(this, "DlqNumberOfMessagesVisibleAlarm", {
                threshold: 1,
                evaluationPeriods: 1,
            });

        this.queue = new Queue(this, "Queue", {
            queueName: `${props.environment}.config-service.workflows.dispatch.v1`,
            enforceSSL: true,
            retentionPeriod: Duration.days(14),
            deadLetterQueue: {
                queue: this.deadLetterQueue,
                maxReceiveCount: 3,
            }
        });
    }
}
