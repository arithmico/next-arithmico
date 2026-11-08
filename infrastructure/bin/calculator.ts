#!/usr/bin/env node
import { getStringParameterOrThrow } from '../lib/utils';
import { CalculatorStack } from '../lib/calculator';
import { App } from 'aws-cdk-lib';
import { CalculatorCertificateStack } from '../lib/calculator-certificate';

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");

const environment = getStringParameterOrThrow("CDK_DEPLOY_ENVIRONMENT");
const artifact = getStringParameterOrThrow("CDK_DEPLOY_ARTIFACT");

const app = new App();

const env = {
    account,
    region
};

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
