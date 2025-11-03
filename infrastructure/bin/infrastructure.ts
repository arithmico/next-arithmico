#!/usr/bin/env node
import { ArtifactsStack } from '../lib/artifacts-stack';
import { getStringParameterOrThrow } from '../lib/utils';
import { ApplicationsStack } from '../lib/applications';
import { App, Stack } from 'aws-cdk-lib';

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");
const refName = getStringParameterOrThrow("GITHUB_REF_NAME");

function getDeploymentEnvironment(): string {
    if (refName === "main") {
        return refName;
    } else if (refName.endsWith("/merge")) {
        return `review-${refName.slice(0, "/merge".length).trim()}`
    }
    throw new Error(`Invalid value for GITHUB_REF_NAME: ${refName}`);
}

function getArtifact(): string {
    if (refName === "main") {
        return getStringParameterOrThrow("APP_VERSION");
    } else if (refName.endsWith("/merge")) {
        return `review-${refName.slice(0, "/merge".length).trim()}`
    }
    throw new Error(`Invalid value for GITHUB_REF_NAME: ${refName}`);
}

const environment = getDeploymentEnvironment();
const artifact = getArtifact();

const app = new App();
let artifacts: Stack | undefined;

if (environment === "main") {
    artifacts = new ArtifactsStack(app, 'Artifacts', {
        env: {
            account,
            region
        }
    });
}

const applications = new ApplicationsStack(app, `applications-${environment}`, {
    env: {
        account,
        region
    },
    artifact,
    environment
})

if (artifacts) {
    applications.addDependency(artifacts)
}
