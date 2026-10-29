import { CorsHttpMethod, DomainName, HttpApi, HttpMethod } from "aws-cdk-lib/aws-apigatewayv2";
import { HttpLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { IFunction } from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";

export interface ApiGatewayProps {
    environment: string;
    lambda: IFunction;
    domainName: DomainName;
}

export class ApiGateway extends HttpApi {
    constructor(scope: Construct, id: string, props: ApiGatewayProps) {
        super(scope, id, {
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
            defaultDomainMapping: {
                domainName: props.domainName
            }
        });
        this.addRoutes({
            path: "/{proxy+}",
            methods: [HttpMethod.ANY],
            integration: new HttpLambdaIntegration("LambdaIntegration", props.lambda),
        });
    }
}