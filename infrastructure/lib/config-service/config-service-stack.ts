import { Stack, StackProps } from "aws-cdk-lib";
import { ISecret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";
import { Database } from "./database";
import { ApiHandler } from "./api-handler";
import { Certificate, CertificateValidation, ICertificate } from "aws-cdk-lib/aws-certificatemanager";
import { ARecord, HostedZone, RecordTarget } from "aws-cdk-lib/aws-route53";
import { ApiGateway } from "./api-gateway";
import { DomainName } from "aws-cdk-lib/aws-apigatewayv2";
import { ApiGatewayv2DomainProperties } from "aws-cdk-lib/aws-route53-targets";
import { GithubAppIdSecret } from "./github-app-secret";
import { WorkflowQueue } from "./workflow-queue";

export interface ConfigServiceStackProps extends StackProps {
    environment: string;
}

export class ConfigServiceStack extends Stack {
    private readonly githubAppSecret: ISecret;
    private readonly database: Database;
    private readonly workflowQueue: WorkflowQueue;
    private readonly apiHandler: ApiHandler;
    private readonly certificate: ICertificate;
    private readonly customDomain: DomainName;
    private readonly apiGateway: ApiGateway;
    private readonly record: ARecord;

    constructor(scope: Construct, id: string, { environment, ...props }: ConfigServiceStackProps) {
        super(scope, id, props);
        const domainName = environment === "main"
            ? "v2.api.arithmico.com"
            : `${environment}.v2.api.arithmico.com`;

        const zone = HostedZone.fromLookup(this, "HostedZone", {
            domainName: "arithmico.com"
        });

        this.githubAppSecret = new GithubAppIdSecret(this, "GithubAppSecret", { environment });

        this.database = new Database(this, "Database", { environment });

        this.workflowQueue = new WorkflowQueue(this, "WorkflowQueue", { environment });

        this.apiHandler = new ApiHandler(this, "ApiHandler", {
            environment,
            githubAppSecret: this.githubAppSecret,
            databaseTable: this.database,
            workflowQueue: this.workflowQueue.queue,
        });

        this.certificate = new Certificate(this, "Certificate", {
            domainName,
            validation: CertificateValidation.fromDns(zone),
        });

        this.customDomain = new DomainName(this, "DomainName", {
            domainName,
            certificate: this.certificate,
        });

        this.apiGateway = new ApiGateway(this, "ApiGateway", {
            domainName: this.customDomain,
            environment,
            lambda: this.apiHandler.lambda
        });

        this.record = new ARecord(this, "DnsRecord", {
            zone,
            recordName: domainName,
            target: RecordTarget.fromAlias(new ApiGatewayv2DomainProperties(
                this.customDomain.regionalDomainName,
                this.customDomain.regionalHostedZoneId
            ))
        });
    }
}
