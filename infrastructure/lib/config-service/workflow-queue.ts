import { Duration } from "aws-cdk-lib";
import { Queue } from "aws-cdk-lib/aws-sqs";
import { Construct } from "constructs";

export interface WorkflowQueueProps {
    environment: string;
}

export class WorkflowQueue extends Construct {
    public readonly queue: Queue;
    public readonly deadLetterQueue: Queue;

    private static generateQueueName(environment: string): string {
        return `${environment}_workflows_dispatch_v1`;
    }

    constructor(scope: Construct, id: string, props: WorkflowQueueProps) {
        super(scope, id)

        this.deadLetterQueue = new Queue(this, "DeadLetterQueue", {
            queueName: `${WorkflowQueue.generateQueueName(props.environment)}_dlq`,
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
            queueName: WorkflowQueue.generateQueueName(props.environment),
            enforceSSL: true,
            retentionPeriod: Duration.days(14),
            deadLetterQueue: {
                queue: this.deadLetterQueue,
                maxReceiveCount: 3,
            }
        });
    }
}
