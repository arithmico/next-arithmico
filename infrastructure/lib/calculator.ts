import { Fn, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";
import { SinglePageApplication } from "./single-page-application";
import { Certificate } from "aws-cdk-lib/aws-certificatemanager";

export type ApplicationsStackProps = StackProps & {
    artifact: string,
    environment: string,
};

export class CalculatorStack extends Stack {
    constructor(scope: Construct, id: string, { artifact, environment, ...props }: ApplicationsStackProps) {
        super(scope, id, props);

        const domainName = environment === "main"
            ? "next.arithmico.com"
            : `${environment}.calculator.arithmico.com`;

        const originAccessIdentityId = Fn.importValue("ArtifactsOriginAccessIdentityId");

        const certificateArn = Fn.getStackOutput(
            `calculator-certificate-${environment}`,
            "CertificateArn",
            "us-east-1",
        );

        const certificate = Certificate.fromCertificateArn(this, "Certificate", certificateArn);

        new SinglePageApplication(this, "Application", {
            artifactPath: `calculator/${artifact}/main/web`,
            domainName,
            originAccessIdentityId,
            certificate
        });
    }
}