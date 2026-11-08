#!/usr/bin/env node
import { App } from "aws-cdk-lib";
import { ArtifactsStack } from "../lib/artifacts-stack";
import { getStringParameterOrThrow } from "../lib/utils";

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");

const env = {
    account,
    region
};

const app = new App();

new ArtifactsStack(app, 'artifacts', {
    env
});
