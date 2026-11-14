import { CfnOutput, Stack, StackProps } from "aws-cdk-lib";
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

        new CfnOutput(this, "ArithmicoComKSKArn", {
            value: this.arithmicoComKsk.keyArn
        });
    }
}
