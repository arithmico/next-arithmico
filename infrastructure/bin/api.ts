#!/usr/bin/env node
import { getStringParameterOrThrow } from '../lib/utils';
import { App } from 'aws-cdk-lib';
import { ApiDataStack } from '../lib/api/api-data-stack';
import { ApiComputeStack } from '../lib/api/api-compute-stack';

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");
const environment = getStringParameterOrThrow("CDK_DEPLOY_ENVIRONMENT");

const app = new App();

const env = {
    account,
    region
};

const apiDataStack = new ApiDataStack(app, `api-data-${environment}`, {
    env,
    environment
});

const apiComputeStack = new ApiComputeStack(app, `api-compute-${environment}`, {
    env,
    environment
});
apiComputeStack.addStackDependency(apiDataStack);
