import { Aws, CfnOutput, Stack, StackProps } from "aws-cdk-lib";
import { PolicyStatement, ServicePrincipal } from "aws-cdk-lib/aws-iam";
import { Key, KeySpec, KeyUsage } from "aws-cdk-lib/aws-kms";
import { Construct } from "constructs";

export class DnssecStack extends Stack {
    private readonly arithmicoComKsk: Key;

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id, props);

        this.arithmicoComKsk = new Key(this, "ArithmicoComKSK", {
            alias: "arithmico-com-dnssec-key",
            keySpec: KeySpec.ECC_NIST_P256,
            keyUsage: KeyUsage.SIGN_VERIFY,
            description: "KMS Key for Route 53 DNSSEC Key-Singning Key (KSK)",
        });

        this.arithmicoComKsk.addToResourcePolicy(new PolicyStatement({
            sid: "Allow Route 53 DNSSEC Service",
            principals: [
                new ServicePrincipal("dnssec-route53.amazonaws.com"),
            ],
            actions: [
                "kms:DescribeKey",
                "kms:GetPublicKey",
                "kms:Sign"
            ],
            resources: ["*"],
            conditions: {
                StringEquals: {
                    "aws:SourceAccount": Aws.ACCOUNT_ID
                }
            }
        }));

        this.arithmicoComKsk.addToResourcePolicy(new PolicyStatement({
            sid: "Allow Route 53 DNSSEC to CreateGrant",
            principals: [
                new ServicePrincipal("dnssec-route53.amazonaws.com")
            ],
            actions: [
                "kms:CreateGrant"
            ],
            resources: ["*"],
            conditions: {
                StringEquals: {
                    "aws:SourceAccount": Aws.ACCOUNT_ID
                },
                Bool: {
                    "kms:GrantIsForAWSResource": true
                }
            }
        }));

        new CfnOutput(this, "ArithmicoComKSKArn", {
            value: this.arithmicoComKsk.keyArn
        });
    }
}
