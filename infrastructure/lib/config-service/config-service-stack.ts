import { Stack, StackProps } from "aws-cdk-lib";
import { ISecret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";
import { Database } from "./database";
import { WebhookSecret } from "./webhook-secret";

export interface ConfigServiceStackProps extends StackProps {
    environment: string;
}

export class ConfigServiceStack extends Stack {
    private readonly webhookSecret: ISecret;
    private readonly database: Database;

    constructor(scope: Construct, id: string, { environment, ...props }: ConfigServiceStackProps) {
        super(scope, id, props);
        this.webhookSecret = new WebhookSecret(this, "WebhookSecert", { environment });
        this.database = new Database(this, "Database", { environment });
    }
}
