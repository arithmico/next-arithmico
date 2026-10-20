import { CfnOutput, RemovalPolicy } from 'aws-cdk-lib';
import { ICertificate } from 'aws-cdk-lib/aws-certificatemanager';
import { Distribution, OriginAccessIdentity } from 'aws-cdk-lib/aws-cloudfront';
import { S3BucketOrigin } from 'aws-cdk-lib/aws-cloudfront-origins';
import { LogGroup } from 'aws-cdk-lib/aws-logs';
import { ARecord, HostedZone, RecordTarget } from 'aws-cdk-lib/aws-route53';
import { CloudFrontTarget } from 'aws-cdk-lib/aws-route53-targets';
import { Bucket } from 'aws-cdk-lib/aws-s3';
import { BucketDeployment, Source } from 'aws-cdk-lib/aws-s3-deployment';
import { Construct } from 'constructs';

export type StaticWebsiteStackProps = {
    domainName: string,
    artifactPath: string,
    originAccessIdentityId: string,
    certificate: ICertificate,
}

export class SinglePageApplication extends Construct {
    constructor(scope: Construct, id: string, { domainName, artifactPath, originAccessIdentityId, certificate, ...props }: StaticWebsiteStackProps) {
        super(scope, id);

        const zone = HostedZone.fromLookup(this, "HostedZone", {
            domainName: "arithmico.com"
        });

        const bucket = Bucket.fromBucketName(this, "Bucket", "arithmico-application-artifacts");

        const originAccessIdentity = OriginAccessIdentity.fromOriginAccessIdentityId(
            this,
            "OriginAccessIdentityId",
            originAccessIdentityId
        );

        const distribution = new Distribution(this, "Distribution", {
            certificate,
            domainNames: [domainName],
            defaultBehavior: {
                origin: S3BucketOrigin.withOriginAccessIdentity(bucket, {
                    originAccessIdentity,
                    originPath: artifactPath,
                }),
            },
            defaultRootObject: "/index.html",
            errorResponses: [
                {
                    httpStatus: 404,
                    responseHttpStatus: 200,
                    responsePagePath: "/index.html"
                },
                {
                    httpStatus: 403,
                    responseHttpStatus: 200,
                    responsePagePath: "/index.html"
                }
            ]
        });

        new CfnOutput(this, `${id}-distribution-url`, {
            value: `https://${distribution.distributionDomainName}`,
        });

        const logGroup = new LogGroup(this, "BucketDeploymentLogGroup", {
            removalPolicy: RemovalPolicy.DESTROY
        });

        new BucketDeployment(this, "BucketDeployment", {
            sources: [Source.asset("../packages/applications/arithmico/dist")],
            destinationKeyPrefix: artifactPath,
            destinationBucket: bucket,
            distribution,
            distributionPaths: ["/*"],
            retainOnDelete: true,
            logGroup
        });

        new CfnOutput(this, 'DistributionId', {
            value: distribution.distributionId,
            key: "DistributionId"
        });

        new ARecord(this, "AliasRecord", {
            zone,
            recordName: domainName,
            target: RecordTarget.fromAlias(
                new CloudFrontTarget(distribution)
            )
        });
    }
}