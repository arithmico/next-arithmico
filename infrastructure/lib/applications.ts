import { Fn, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";
import { StaticWebsiteDeployment } from "./static-website-deployment";



export type ApplicationsStackProps = StackProps & {
    artifact: string
};



export class ApplicationsStack extends Stack {
    constructor(scope: Construct, id: string, { artifact, ...props }: ApplicationsStackProps) {
        super(scope, id, props);

        const originAccessIdentityId = Fn.importValue("ArtifactsOriginAccessIdentityId");

        new StaticWebsiteDeployment(this, "calculator", {
            artifactPath: `calculator/${artifact}/main/web`,
            domainName: "next.arithmico.com",
            originAccessIdentityId
        })
    }
}