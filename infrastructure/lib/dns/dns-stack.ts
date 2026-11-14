import { Stack, StackProps } from "aws-cdk-lib";
import { PublicHostedZone } from "aws-cdk-lib/aws-route53";
import { Construct } from "constructs";

export class DnsStack extends Stack {
    private readonly zone: PublicHostedZone;

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id);

        this.zone = new PublicHostedZone(this, "HostedZone", {
            zoneName: "arithmico.com",
            comment: "Root dns zone of arithmico.",
        });
    }
}
