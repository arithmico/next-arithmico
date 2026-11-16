import { Duration, RemovalPolicy } from "aws-cdk-lib";
import { ComparisonOperator, TreatMissingData } from "aws-cdk-lib/aws-cloudwatch";
import { ITableV2 } from "aws-cdk-lib/aws-dynamodb";
import { PolicyStatement } from "aws-cdk-lib/aws-iam";
import { Architecture } from "aws-cdk-lib/aws-lambda";
import { ILogGroup, LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import { Schedule, ScheduleExpression } from "aws-cdk-lib/aws-scheduler";
import { LambdaInvoke } from "aws-cdk-lib/aws-scheduler-targets";
import { ISecret } from "aws-cdk-lib/aws-secretsmanager";
import { IQueue } from "aws-cdk-lib/aws-sqs";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";
import * as path from "path";

export interface WorkflowDispatcherProps {
    environment: string;
    workflowQueue: IQueue;
    githubAppSecret: ISecret;
    databaseTable: ITableV2;
}

export class WorkflowDispatcher extends Construct {
    public readonly logGroup: ILogGroup;
    public readonly lambda: RustFunction;
    public readonly schedule: Schedule;

    constructor(scope: Construct, id: string, props: WorkflowDispatcherProps) {
        super(scope, id);

        this.logGroup = new LogGroup(this, "LogGroup", {
            logGroupName: `/workflow-dispatcher/${props.environment}`,
            retention: RetentionDays.THREE_MONTHS,
            removalPolicy: RemovalPolicy.DESTROY,
        });

        this.lambda = new RustFunction(this, "Function", {
            manifestPath: path.join(__dirname, "../../../packages/applications/workflow-dispatcher/Cargo.toml"),
            architecture: Architecture.ARM_64,
            environment: {
                GITHUB_APP_SECRET_ARN: props.githubAppSecret.secretArn,
                DYNAMODB_TABLE_NAME: props.databaseTable.tableName,
                WORKFLOW_QUEUE_URL: props.workflowQueue.queueUrl,
            },
            runtime: "provided.al2023",
            logGroup: this.logGroup,
            reservedConcurrentExecutions: 1,
            timeout: Duration.seconds(10),
        });

        this.lambda
            .metricThrottles()
            .createAlarm(this, "ThrottleAlarm", {
                threshold: 1,
                evaluationPeriods: 1
            });

        this.lambda
            .metricErrors()
            .createAlarm(this, "ErrorsAlarm", {
                threshold: 1,
                evaluationPeriods: 1
            });

        this.lambda
            .metricDuration({
                statistic: "p99",
            })
            .createAlarm(this, "DurationP99Alarm", {
                threshold: 5000,
                evaluationPeriods: 3,
                datapointsToAlarm: 2,
                comparisonOperator: ComparisonOperator.GREATER_THAN_THRESHOLD,
                treatMissingData: TreatMissingData.NOT_BREACHING,
            });

        this.schedule = new Schedule(this, "FixedRateSchedule", {
            schedule: ScheduleExpression.rate(Duration.minutes(5)),
            target: new LambdaInvoke(this.lambda),
            description: "Trigger the workflow dispatcher at a fixed rate",
        });

        this.lambda.addToRolePolicy(new PolicyStatement({
            sid: "AllowAccessToGithubAppSecret",
            actions: [
                "secretsmanager:GetSecretValue",
                "secretsmanager:DescribeSecret",
            ],
            resources: [props.githubAppSecret.secretArn]
        }));
        this.lambda.addToRolePolicy(new PolicyStatement({
            sid: "AllowDynamodbAccess",
            actions: [
                "dynamodb:BatchGetItem",
                "dynamodb:GetRecords",
                "dynamodb:GetShardIterator",
                "dynamodb:Query",
                "dynamodb:GetItem",
                "dynamodb:Scan",
                "dynamodb:BatchWriteItem",
                "dynamodb:PutItem",
                "dynamodb:UpdateItem",
                "dynamodb:DeleteItem",
            ],
            resources: [props.databaseTable.tableArn]
        }));
        this.lambda.addToRolePolicy(new PolicyStatement({
            sid: "AllowReceivingMessagesFromTheSqsQueue",
            actions: [
                "sqs:ChangeMessageVisibility",
                "sqs:DeleteMessage",
                "sqs:ReceiveMessage",
                "sqs:GetQueueAttributes",
                "sqs:GetQueueUrl"
            ],
            resources: [props.workflowQueue.queueArn],
        }));
    }
}