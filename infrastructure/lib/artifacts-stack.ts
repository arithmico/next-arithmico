import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';

export class ArtifactsStack extends cdk.Stack {
  private originAccessIdentityId: string;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const bucket = new cdk.aws_s3.Bucket(this, "Bucket", {
      bucketName: "arithmico-application-artifacts",
      blockPublicAccess: cdk.aws_s3.BlockPublicAccess.BLOCK_ALL,
      encryption: cdk.aws_s3.BucketEncryption.S3_MANAGED,
      enforceSSL: true,
      versioned: true,
      removalPolicy: cdk.RemovalPolicy.RETAIN
    });

    const originAccessIdentity = new cdk.aws_cloudfront.OriginAccessIdentity(this, 'OriginAccessIdentity');
    bucket.grantRead(originAccessIdentity);
    this.originAccessIdentityId = originAccessIdentity.originAccessIdentityId;
  }

  public getOriginAccessIdentityId(): string {
    return this.originAccessIdentityId;
  }
}
