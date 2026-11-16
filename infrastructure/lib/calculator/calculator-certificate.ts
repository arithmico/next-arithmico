import { CfnOutput, Stack, StackProps } from "aws-cdk-lib";
import { Certificate, CertificateValidation } from "aws-cdk-lib/aws-certificatemanager";
import { HostedZone } from "aws-cdk-lib/aws-route53";
import { Construct } from "constructs";
import { getCalculatorHostname } from "./calculator-hostname";

export interface CalculatorCertificateProps extends StackProps {
    environment: string,
}
export class CalculatorCertificateStack extends Stack {
    private readonly certificate: Certificate;

    constructor(scope: Construct, id: string, { environment, ...props }: CalculatorCertificateProps) {
        super(scope, id, props);

        const domainName = getCalculatorHostname(environment);

        const zone = HostedZone.fromLookup(this, "HostedZone", {
            domainName: "arithmico.com",
        });

        this.certificate = new Certificate(this, "Certificate", {
            domainName,
            validation: CertificateValidation.fromDns(zone)
        });

        new CfnOutput(this, "CertificateArn", {
            value: this.certificate.certificateArn
        });
    }
}