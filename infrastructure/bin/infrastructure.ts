#!/usr/bin/env node
import { ArtifactsStack } from '../lib/artifacts-stack';
import { getStringParameterOrThrow } from '../lib/utils';
import { ApplicationsStack } from '../lib/applications';
import { App, Stack } from 'aws-cdk-lib';

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

const applications = new ApplicationsStack(app, `applications-${environment}`, {
    env,
    artifact,
    environment
})

if (artifacts) {
    applications.addStackDependency(artifacts)
}
