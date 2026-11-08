#!/usr/bin/env node
import { getStringParameterOrThrow } from '../lib/utils';
import { App } from 'aws-cdk-lib';
import { ConfigServiceStack } from '../lib/config-service/config-service-stack';

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");
const environment = getStringParameterOrThrow("CDK_DEPLOY_ENVIRONMENT");

const app = new App();

const env = {
    account,
    region
};

new ConfigServiceStack(app, `config-service-${environment}`, {
    env,
    environment
});
