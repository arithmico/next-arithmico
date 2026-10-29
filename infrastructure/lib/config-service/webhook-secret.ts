import { Secret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

export interface WebhookSecretProps {
    environment: string;
}

export class WebhookSecret extends Secret {
    constructor(scope: Construct, id: string, { environment }: WebhookSecretProps) {
        super(scope, id, {
            secretName: `/config-service/${environment}/webhook-secret`,
            generateSecretString: {
                excludePunctuation: true,
                passwordLength: 32
            }
        });
    }
}
