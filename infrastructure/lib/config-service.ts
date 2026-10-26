import { Stack, StackProps } from "aws-cdk-lib";
import { ISecret, Secret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

export interface ConfigServiceStackProps extends StackProps {
    environment: string;
}

export class ConfigServiceStack extends Stack {
    private readonly webhookSecret: ISecret;

    constructor(scope: Construct, id: string, { environment, ...props }: ConfigServiceStackProps) {
        super(scope, id, props);

        this.webhookSecret = new Secret(this, "WebhookSecret", {
            secretName: `/config-service/${environment}/webhook-secret`,
            generateSecretString: {
                excludePunctuation: true,
                passwordLength: 32
            }
        });
    }
}
