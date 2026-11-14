import { Fn, Stack, StackProps } from "aws-cdk-lib";
import { Key } from "aws-cdk-lib/aws-kms";
import { PublicHostedZone } from "aws-cdk-lib/aws-route53";
import { Construct } from "constructs";

export class DnsStack extends Stack {
    private readonly zone: PublicHostedZone;

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id);

        const keyArn = Fn.getStackOutput("dnssec", "ArithmicoComKSKArn", "us-east-1");

        const key = Key.fromKeyArn(this, "KSK", keyArn);

        this.zone = new PublicHostedZone(this, "HostedZone", {
            zoneName: "arithmico.com",
            comment: "Root dns zone of arithmico.",
        });

        this.zone.enableDnssec({
            kmsKey: key,
            keySigningKeyName: "arithmico_com_ksk_v1"
        });
    }
}
