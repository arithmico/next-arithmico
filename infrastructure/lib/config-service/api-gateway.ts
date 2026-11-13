import { CorsHttpMethod, DomainName, HttpApi, HttpMethod, IpAddressType } from "aws-cdk-lib/aws-apigatewayv2";
import { HttpLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Certificate, CertificateValidation, ICertificate } from "aws-cdk-lib/aws-certificatemanager";
import { IFunction } from "aws-cdk-lib/aws-lambda";
import { AaaaRecord, ARecord, IHostedZone, RecordTarget } from "aws-cdk-lib/aws-route53";
import { ApiGatewayv2DomainProperties } from "aws-cdk-lib/aws-route53-targets";
import { Construct } from "constructs";

export interface ApiGatewayProps {
    environment: string;
    lambda: IFunction;
    domainName: string;
    zone: IHostedZone;
}

export class ApiGateway extends Construct {
    public readonly httpApi: HttpApi;
    public readonly certificate: ICertificate;
    public readonly customDomain: DomainName;
    public readonly ipv4Record: ARecord;
    public readonly ipv6Record: AaaaRecord;

    constructor(scope: Construct, id: string, props: ApiGatewayProps) {
        super(scope, id);

        this.certificate = new Certificate(this, "Certificate", {
            domainName: props.domainName,
            validation: CertificateValidation.fromDns(props.zone),
        });

        this.customDomain = new DomainName(this, "DomainName", {
            domainName: props.domainName,
            certificate: this.certificate,
        });

        this.httpApi = new HttpApi(this, "HttpApi", {
            apiName: `config-service-${props.environment}`,
            corsPreflight: {
                allowOrigins: ['*'],
                allowMethods: [
                    CorsHttpMethod.GET,
                    CorsHttpMethod.POST,
                    CorsHttpMethod.OPTIONS,
                ],
                allowHeaders: [
                    "Content-Type",
                    "Authorization",
                    "X-Hub-Signature-256"
                ]
            },
            ipAddressType: IpAddressType.DUAL_STACK,
            disableExecuteApiEndpoint: true,
            defaultDomainMapping: {
                domainName: this.customDomain,
            }
        });

        this.httpApi.addRoutes({
            path: "/{proxy+}",
            methods: [HttpMethod.ANY],
            integration: new HttpLambdaIntegration("LambdaIntegration", props.lambda),
        });

        const target = RecordTarget.fromAlias(new ApiGatewayv2DomainProperties(
            this.customDomain.regionalDomainName,
            this.customDomain.regionalHostedZoneId
        ));

        this.ipv4Record = new ARecord(this, "Ipv4Record", {
            zone: props.zone,
            recordName: props.domainName,
            target
        });

        this.ipv6Record = new AaaaRecord(this, "Ipv6Record", {
            zone: props.zone,
            recordName: props.domainName,
            target
        });
    }
}
