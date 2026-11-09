import { RemovalPolicy } from "aws-cdk-lib";
import { ComparisonOperator, TreatMissingData } from "aws-cdk-lib/aws-cloudwatch";
import { ITableV2 } from "aws-cdk-lib/aws-dynamodb";
import { Architecture } from "aws-cdk-lib/aws-lambda";
import { ILogGroup, LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import { ISecret } from "aws-cdk-lib/aws-secretsmanager";
import { IQueue } from "aws-cdk-lib/aws-sqs";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";
import * as path from "path";

const GITHUB_OIDC_CONFIGURATION_URI = "https://token.actions.githubusercontent.com/.well-known/openid-configuration";

export interface ApiHandlerProps {
    githubAppSecret: ISecret;
    databaseTable: ITableV2;
    environment: string;
    workflowQueue: IQueue;
}

export class ApiHandler extends Construct {
    public readonly logGroup: ILogGroup;
    public readonly lambda: RustFunction;

    constructor(scope: Construct, id: string, props: ApiHandlerProps) {
        super(scope, id);

        this.logGroup = new LogGroup(this, "LogGroup", {
            logGroupName: `/api-handler/${props.environment}`,
            retention: RetentionDays.THREE_MONTHS,
            removalPolicy: RemovalPolicy.DESTROY,
        });

        this.lambda = new RustFunction(this, "Function", {
            manifestPath: path.join(__dirname, "../../../packages/applications/config-service/Cargo.toml"),
            architecture: Architecture.ARM_64,
            environment: {
                GITHUB_APP_SECRET_ARN: props.githubAppSecret.secretArn,
                DYNAMODB_TABLE_NAME: props.databaseTable.tableName,
                GITHUB_OIDC_CONFIGURATION_URI,
                WORKFLOW_QUEUE_URL: props.workflowQueue.queueUrl,
            },
            runtime: "provided.al2023",
            binaryName: "bootstrap",
            logGroup: this.logGroup,
            reservedConcurrentExecutions: 50,
        });

        this.lambda
            .metricThrottles()
            .createAlarm(this, "ThrottleAlarm", {
                threshold: 1,
                evaluationPeriods: 3
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
                threshold: 10000,
                evaluationPeriods: 3,
                datapointsToAlarm: 2,
                comparisonOperator: ComparisonOperator.GREATER_THAN_THRESHOLD,
                treatMissingData: TreatMissingData.NOT_BREACHING,
            });

        props.githubAppSecret.grantRead(this.lambda);
        props.databaseTable.grantReadWriteData(this.lambda);
        props.workflowQueue.grantSendMessages(this.lambda);
    }
}
