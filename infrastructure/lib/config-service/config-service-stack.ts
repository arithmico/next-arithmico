import { Stack, StackProps } from "aws-cdk-lib";
import { ISecret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";
import { Database } from "./database";
import { WebhookSecret } from "./webhook-secret";
import { ApiHandler } from "./api-handler";
import { Certificate, CertificateValidation, ICertificate } from "aws-cdk-lib/aws-certificatemanager";
import { HostedZone } from "aws-cdk-lib/aws-route53";
import { ApiGateway } from "./api-gateway";
import { DomainName } from "aws-cdk-lib/aws-apigatewayv2";

export interface ConfigServiceStackProps extends StackProps {
    environment: string;
}

export class ConfigServiceStack extends Stack {
    private readonly webhookSecret: ISecret;
    private readonly database: Database;
    private readonly apiHandler: ApiHandler;
    private readonly certificate: ICertificate;
    private readonly apiGateway: ApiGateway;

    constructor(scope: Construct, id: string, { environment, ...props }: ConfigServiceStackProps) {
        super(scope, id, props);
        const domainName = environment === "main"
            ? "v2.api.arithmico.com"
            : `${environment}.v2.api.arithmico.com`;

        const zone = HostedZone.fromLookup(this, "HostedZone", {
            domainName
        });

        this.webhookSecret = new WebhookSecret(this, "WebhookSecert", { environment });
        this.database = new Database(this, "Database", { environment });
        this.apiHandler = new ApiHandler(this, "ApiHandler", {
            environment,
            webhookSecret: this.webhookSecret,
        });
        this.certificate = new Certificate(this, "Certificate", {
            domainName,
            validation: CertificateValidation.fromDns(zone),
        });
        this.apiGateway = new ApiGateway(this, "ApiGateway", {
            domainName: new DomainName(this, "DomainName", {
                domainName,
                certificate: this.certificate,
            }),
            environment,
            lambda: this.apiHandler.lambda
        });
    }
}
