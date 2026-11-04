import { Secret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

export interface GithubAppIdSecretProps {
    environment: string;
}

export class GithubAppIdSecret extends Secret {
    constructor(scope: Construct, id: string, { environment }: GithubAppIdSecretProps) {
        super(scope, id, {
            secretName: `/config-service/${environment}/github-app`,
            description: "webhook secret and github client id"
        });
    }
}
