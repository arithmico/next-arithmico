import * as cdk from 'aws-cdk-lib';
import { BucketDeployment, Source } from 'aws-cdk-lib/aws-s3-deployment';
import { Construct } from 'constructs';

export type StaticWebsiteStackProps = {
    domainName: string,
    artifactPath: string,
    originAccessIdentityId: string
}

export class StaticWebsiteDeployment extends Construct {
    constructor(scope: Construct, id: string, { domainName, artifactPath, originAccessIdentityId, ...props }: StaticWebsiteStackProps) {
        super(scope, id);

        const zone = cdk.aws_route53.HostedZone.fromLookup(this, "HostedZone", {
            domainName: "arithmico.com"
        });

        const bucket = cdk.aws_s3.Bucket.fromBucketName(this, "Bucket", "arithmico-application-artifacts");

        const originAccessIdentity = cdk.aws_cloudfront.OriginAccessIdentity.fromOriginAccessIdentityId(
            this,
            "OriginAccessIdentityId",
            originAccessIdentityId
        );

        const certificate = new cdk.aws_certificatemanager.DnsValidatedCertificate(this, "Certificate", {
            domainName,
            hostedZone: zone,
            region: "us-east-1"
        });

        const distribution = new cdk.aws_cloudfront.Distribution(this, "Distribution", {
            certificate,
            domainNames: [domainName],
            defaultBehavior: {
                origin: cdk.aws_cloudfront_origins.S3BucketOrigin.withOriginAccessIdentity(bucket, {
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

        new BucketDeployment(this, "BucketDeployment", {
            sources: [Source.asset("../packages/applications/arithmico/dist")],
            destinationKeyPrefix: artifactPath,
            destinationBucket: bucket,
            distribution,
            distributionPaths: ["/*"]
        });

        new cdk.CfnOutput(this, 'DistributionId', {
            value: distribution.distributionId,
            key: "DistributionId"
        });

        new cdk.aws_route53.ARecord(this, "AliasRecord", {
            zone,
            recordName: domainName,
            target: cdk.aws_route53.RecordTarget.fromAlias(
                new cdk.aws_route53_targets.CloudFrontTarget(distribution)
            )
        });
    }
}