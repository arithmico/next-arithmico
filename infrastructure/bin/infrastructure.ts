#!/usr/bin/env node
import { ArtifactsStack } from '../lib/artifacts-stack';
import { getStringParameterOrThrow } from '../lib/utils';
import { CalculatorStack } from '../lib/calculator';
import { App, Stack } from 'aws-cdk-lib';
import { CalculatorCertificateStack } from '../lib/calculator-certificate';

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");

const environment = getStringParameterOrThrow("CDK_DEPLOY_ENVIRONMENT");
const artifact = getStringParameterOrThrow("CDK_DEPLOY_ARTIFACT");

const app = new App();
let artifacts: Stack | undefined;

const env = {
    account,
    region
};

if (environment === "main") {
    artifacts = new ArtifactsStack(app, 'Artifacts', {
        env
    });
}

const calculatorCertificate = new CalculatorCertificateStack(app, `calculator-certificate-${environment}`, {
    env: {
        account: env.account,
        region: "us-east-1"
    },
    environment,
});

const calculator = new CalculatorStack(app, `calculator-${environment}`, {
    env,
    artifact,
    environment,
});
calculator.addStackDependency(calculatorCertificate);

if (artifacts) {
    calculator.addStackDependency(artifacts)
}
