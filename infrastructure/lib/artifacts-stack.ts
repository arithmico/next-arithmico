import { CfnOutput, RemovalPolicy, Stack, StackProps } from 'aws-cdk-lib';
import { OriginAccessIdentity } from 'aws-cdk-lib/aws-cloudfront';
import { BlockPublicAccess, Bucket, BucketEncryption } from 'aws-cdk-lib/aws-s3';
import { Construct } from 'constructs';

export class ArtifactsStack extends Stack {

  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const bucket = new Bucket(this, "Bucket", {
      bucketName: "arithmico-application-artifacts",
      blockPublicAccess: BlockPublicAccess.BLOCK_ALL,
      encryption: BucketEncryption.S3_MANAGED,
      enforceSSL: true,
      versioned: true,
      removalPolicy: RemovalPolicy.RETAIN
    });

    const originAccessIdentity = new OriginAccessIdentity(this, 'OriginAccessIdentity');
    bucket.grantRead(originAccessIdentity);

    new CfnOutput(this, "OriginAccessIdentityOutput", {
      value: originAccessIdentity.originAccessIdentityId,
      description: "Id of the OriginAccessIdentity for the artifacts bucket",
      exportName: "ArtifactsOriginAccessIdentityId"
    })
  }

}
