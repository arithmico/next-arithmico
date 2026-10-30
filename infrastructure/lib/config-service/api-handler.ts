import { RemovalPolicy } from "aws-cdk-lib";
import { ITableV2 } from "aws-cdk-lib/aws-dynamodb";
import { Architecture } from "aws-cdk-lib/aws-lambda";
import { ILogGroup, LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import { ISecret } from "aws-cdk-lib/aws-secretsmanager";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";
import * as path from "path";

export interface ApiHandlerProps {
    webhookSecret: ISecret;
    databaseTable: ITableV2;
    environment: string;
}

export class ApiHandler extends Construct {
    public readonly logGroup: ILogGroup;
    public readonly lambda: RustFunction;

    constructor(scope: Construct, id: string, props: ApiHandlerProps) {
        super(scope, id);

        this.logGroup = new LogGroup(this, "LogGroup", {
            logGroupName: `/config-service/${props.environment}/api-handler`,
            retention: RetentionDays.THREE_MONTHS,
            removalPolicy: RemovalPolicy.DESTROY,
        });

        this.lambda = new RustFunction(this, "Function", {
            manifestPath: path.join(__dirname, "../../../packages/applications/config-service/Cargo.toml"),
            architecture: Architecture.ARM_64,
            environment: {
                WEBHOOK_SECRET_ARN: props.webhookSecret.secretArn,
                DYNAMODB_TABLE_NAME: props.databaseTable.tableName,
            },
            runtime: "provided.al2023",
            binaryName: "bootstrap",
            logGroup: this.logGroup,
            reservedConcurrentExecutions: 50,
        });

        props.webhookSecret.grantRead(this.lambda);
        props.databaseTable.grantReadWriteData(this.lambda);
    }
}
