import { Stack, StackProps } from "aws-cdk-lib";
import { ISecret } from "aws-cdk-lib/aws-secretsmanager";
import { Database } from "./database";
import { WorkflowQueue } from "./workflow-queue";
import { Construct } from "constructs";
import { GithubAppSecret } from "./github-app-secret";

export interface ApiDataStackProps extends StackProps {
    environment: string;
}

export class ApiDataStack extends Stack {
    private readonly githubAppSecret: ISecret;
    private readonly database: Database;
    private readonly workflowQueue: WorkflowQueue;

    constructor(scope: Construct, id: string, { environment, ...props }: ApiDataStackProps) {
        super(scope, id, props);

        this.githubAppSecret = new GithubAppSecret(this, "GithubAppSecret", { environment });

        this.database = new Database(this, "Database", { environment });

        this.workflowQueue = new WorkflowQueue(this, "WorkflowQueue", { environment });
    }
}
