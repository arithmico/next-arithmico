import { CfnOutput, Fn, RemovalPolicy } from "aws-cdk-lib";
import { ISecret, Secret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

export interface GithubAppIdSecretProps {
    environment: string;
}

export class GithubAppSecret extends Secret {
    constructor(scope: Construct, id: string, { environment }: GithubAppIdSecretProps) {
        super(scope, id, {
            secretName: GithubAppSecret.getSecretName(environment),
            description: "Contains webhook secret, github client id and private key.",
            removalPolicy: RemovalPolicy.RETAIN,
        });

        new CfnOutput(this, "GithubAppSecretArn", {
            key: "GithubAppSecretArn",
            value: this.secretArn,
        });
    }

    private static getSecretName(environment: string): string {
        return `/api/${environment}/github-app`;
    }

    public static lookup(scope: Construct, id: string, environment: string): ISecret {
        const secretArn = Fn.getStackOutput(`api-data-${environment}`, "GithubAppSecretArn");
        return Secret.fromSecretCompleteArn(scope, id, secretArn);
    }
}
