import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";
import { Database } from "./database";
import { ApiHandler } from "./api-handler";
import { HostedZone } from "aws-cdk-lib/aws-route53";
import { ApiGateway } from "./api-gateway";
import { GithubAppSecret } from "./github-app-secret";
import { WorkflowQueue } from "./workflow-queue";
import { WorkflowDispatcher } from "./workflow-dispatcher";

export interface ApiComputeStackProps extends StackProps {
    environment: string;
}

export class ApiComputeStack extends Stack {
    private readonly apiHandler: ApiHandler;
    private readonly workflowDispatcher: WorkflowDispatcher;
    private readonly apiGateway: ApiGateway;

    constructor(scope: Construct, id: string, { environment, ...props }: ApiComputeStackProps) {
        super(scope, id, props);
        const domainName = environment === "production"
            ? "v2.api.arithmico.com"
            : `${environment}.v2.api.arithmico.com`;

        const zone = HostedZone.fromLookup(this, "HostedZone", {
            domainName: "arithmico.com"
        });

        const workflowQueue = WorkflowQueue.lookup(this, "WorkflowQueue", environment);
        const databaseTable = Database.lookup(this, "Database", environment);
        const githubAppSecret = GithubAppSecret.lookup(this, "GithubAppSecret", environment);

        this.apiHandler = new ApiHandler(this, "ApiHandler", {
            environment,
            githubAppSecret,
            databaseTable,
            workflowQueue,
        });

        this.workflowDispatcher = new WorkflowDispatcher(this, "WorkflowDispatcher", {
            environment,
            githubAppSecret,
            databaseTable,
            workflowQueue,
        });

        this.apiGateway = new ApiGateway(this, "ApiGateway", {
            zone,
            domainName,
            environment,
            lambda: this.apiHandler.lambda
        });
    }
}
