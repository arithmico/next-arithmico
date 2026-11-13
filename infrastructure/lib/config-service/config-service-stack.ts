import { Stack, StackProps } from "aws-cdk-lib";
import { ISecret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";
import { Database } from "./database";
import { ApiHandler } from "./api-handler";
import { HostedZone } from "aws-cdk-lib/aws-route53";
import { ApiGateway } from "./api-gateway";
import { GithubAppIdSecret } from "./github-app-secret";
import { WorkflowQueue } from "./workflow-queue";
import { WorkflowDispatcher } from "./workflow-dispatcher";

export interface ConfigServiceStackProps extends StackProps {
    environment: string;
}

export class ConfigServiceStack extends Stack {
    private readonly githubAppSecret: ISecret;
    private readonly database: Database;
    private readonly workflowQueue: WorkflowQueue;
    private readonly apiHandler: ApiHandler;
    private readonly workflowDispatcher: WorkflowDispatcher;
    private readonly apiGateway: ApiGateway;

    constructor(scope: Construct, id: string, { environment, ...props }: ConfigServiceStackProps) {
        super(scope, id, props);
        const domainName = environment === "main"
            ? "v2.api.arithmico.com"
            : `${environment}.v2.api.arithmico.com`;

        const zone = HostedZone.fromLookup(this, "HostedZone", {
            domainName: "arithmico.com"
        });

        this.githubAppSecret = new GithubAppIdSecret(this, "GithubAppSecret", { environment });

        this.database = new Database(this, "Database", { environment });

        this.workflowQueue = new WorkflowQueue(this, "WorkflowQueue", { environment });

        this.apiHandler = new ApiHandler(this, "ApiHandler", {
            environment,
            githubAppSecret: this.githubAppSecret,
            databaseTable: this.database,
            workflowQueue: this.workflowQueue.queue,
        });

        this.workflowDispatcher = new WorkflowDispatcher(this, "WorkflowDispatcher", {
            environment,
            githubAppSecret: this.githubAppSecret,
            databaseTable: this.database,
            workflowQueue: this.workflowQueue.queue,
        });

        this.apiGateway = new ApiGateway(this, "ApiGateway", {
            zone,
            domainName,
            environment,
            lambda: this.apiHandler.lambda
        });
    }
}
