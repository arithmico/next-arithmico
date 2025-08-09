#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { ArtifactsStack } from '../lib/artifacts-stack';
import { getStringParameterOrThrow } from '../lib/utils';
import { StaticWebsiteStack } from '../lib/static-website-stack';

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");
const version = getStringParameterOrThrow("APP_VERSION");

const app = new cdk.App();

const artifactsStack = new ArtifactsStack(app, 'Artifacts', {
    env: {
        account,
        region
    }
});

const calculatorProductionStack = new StaticWebsiteStack(app, "CalculatorProduction", {
    env: {
        account,
        region
    },
    artifactPath: `calculator/${version}/main/web`,
    domainName: "next.arithmico.com"
});
calculatorProductionStack.addDependency(artifactsStack);
