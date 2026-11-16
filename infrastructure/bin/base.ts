#!/usr/bin/env node
import { App } from "aws-cdk-lib";
import { ArtifactsStack } from "../lib/artifacts-stack";
import { getStringParameterOrThrow } from "../lib/utils";
import { DnsStack } from "../lib/dns/dns-stack";
import { DnssecStack } from "../lib/dnssec/dnssec-stack";

const account = getStringParameterOrThrow("AWS_ACCOUNT_ID");
const region = getStringParameterOrThrow("AWS_REGION");

const env = {
    account,
    region
};

const app = new App();

new ArtifactsStack(app, "artifacts", {
    env
});

const dnssecStack = new DnssecStack(app, "dnssec", {
    env: {
        account: env.account,
        region: "us-east-1"
    },
    terminationProtection: true,
});

const dnsStack = new DnsStack(app, "dns", {
    env,
    terminationProtection: true,
});
dnsStack.addStackDependency(dnssecStack);

