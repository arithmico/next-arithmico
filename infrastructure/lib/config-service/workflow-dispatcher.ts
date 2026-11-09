import { RemovalPolicy } from "aws-cdk-lib";
import { ITableV2 } from "aws-cdk-lib/aws-dynamodb";
import { Architecture } from "aws-cdk-lib/aws-lambda";
import { ILogGroup, LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
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
        });

        // TODO: scheduled execution with event bridge

        props.githubAppSecret.grantRead(this.lambda);
        props.databaseTable.grantReadWriteData(this.lambda);
        props.workflowQueue.grantConsumeMessages(this.lambda);
    }
}