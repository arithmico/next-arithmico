import { CfnOutput, Duration, RemovalPolicy, Stack, StackProps } from 'aws-cdk-lib';
import { OriginAccessIdentity } from 'aws-cdk-lib/aws-cloudfront';
import { BlockPublicAccess, Bucket, BucketEncryption } from 'aws-cdk-lib/aws-s3';
import { Construct } from 'constructs';

export class ArtifactsStack extends Stack {
  private readonly artifacts: Bucket;
  private readonly artifactsOriginAccessIdentity: OriginAccessIdentity;
  private readonly sharedCompilationCache: Bucket;

  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    this.artifacts = new Bucket(this, "Bucket", {
      bucketName: "arithmico-application-artifacts",
      blockPublicAccess: BlockPublicAccess.BLOCK_ALL,
      encryption: BucketEncryption.S3_MANAGED,
      enforceSSL: true,
      versioned: true,
      removalPolicy: RemovalPolicy.RETAIN,
    });

    this.artifactsOriginAccessIdentity = new OriginAccessIdentity(this, 'OriginAccessIdentity');
    this.artifacts.grantRead(this.artifactsOriginAccessIdentity);

    this.sharedCompilationCache = new Bucket(this, "SharedCompilationCache", {
      bucketName: "arithmico-shared-compilation-cache",
      blockPublicAccess: BlockPublicAccess.BLOCK_ALL,
      encryption: BucketEncryption.S3_MANAGED,
      enforceSSL: true,
      versioned: true,
      removalPolicy: RemovalPolicy.RETAIN,
      lifecycleRules: [
        {
          id: "ExpireObjectsAfter10Days",
          enabled: true,
          expiration: Duration.days(10),
        },
      ],
    });

    new CfnOutput(this, "OriginAccessIdentityId", {
      value: this.artifactsOriginAccessIdentity.originAccessIdentityId,
      description: "Id of the OriginAccessIdentity for the artifacts bucket",
    });
  }
}
