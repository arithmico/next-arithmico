#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { ArtifactsStack } from '../lib/artifacts-stack';
import { getStringParameterOrThrow } from '../lib/utils';

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");

const app = new cdk.App();

new ArtifactsStack(app, 'Artifacts', {
    env: {
        account,
        region
    }
});
