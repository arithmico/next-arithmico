import { Fn, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";
import { StaticWebsiteDeployment } from "./static-website-deployment";

export type ApplicationsStackProps = StackProps & {
    artifact: string,
    environment: string,
};

export class ApplicationsStack extends Stack {
    constructor(scope: Construct, id: string, { artifact, environment, ...props }: ApplicationsStackProps) {
        super(scope, id, props);

        const domainName = environment === "main"
            ? "next.arithmico.com"
            : `${environment}.calculator.arithmico.com`;

        const originAccessIdentityId = Fn.importValue("ArtifactsOriginAccessIdentityId");

        new StaticWebsiteDeployment(this, "calculator", {
            artifactPath: `calculator/${artifact}/main/web`,
            domainName,
            originAccessIdentityId
        })
    }
}