import { Key, KeySpec, KeyUsage } from "aws-cdk-lib/aws-kms";
import { Construct } from "constructs";

export interface JwtKeyProps {
    environment: string;
}

export class JwtKey extends Key {
    constructor(scope: Construct, id: string, props: JwtKeyProps) {
        super(scope, id, {
            description: "Sign and verify json web tokens.",
            alias: `config-service/${props.environment}`,
            keySpec: KeySpec.RSA_4096,
            keyUsage: KeyUsage.SIGN_VERIFY,
        });
    }
}