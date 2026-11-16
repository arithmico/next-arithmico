import { CfnOutput, Fn } from "aws-cdk-lib";
import { AttributeType, Billing, ITableV2, ProjectionType, TableV2 } from "aws-cdk-lib/aws-dynamodb";
import { Construct } from "constructs";

export interface DatabaseProps {
    environment: string;
}

export class Database extends TableV2 {
    constructor(scope: Construct, id: string, { environment }: DatabaseProps) {
        super(scope, id, {
            tableName: `api-${environment}`,
            partitionKey: {
                name: "PK",
                type: AttributeType.STRING,
            },
            sortKey: {
                name: "SK",
                type: AttributeType.STRING,
            },
            billing: Billing.onDemand(),
        });
        this.addGlobalSecondaryIndex({
            indexName: "GSI1",
            partitionKey: {
                name: "GSI1PK",
                type: AttributeType.STRING,
            },
            sortKey: {
                name: "GSI1SK",
                type: AttributeType.STRING,
            },
            projectionType: ProjectionType.ALL,
        });

        new CfnOutput(this, "DatabaseTableArn", {
            key: "DatabaseTableArn",
            value: this.tableArn
        });
    }

    public static lookup(scope: Construct, id: string, environment: string): ITableV2 {
        const tableArn = Fn.getStackOutput(`api-data-${environment}`, "DatabaseTableArn");
        return TableV2.fromTableArn(scope, id, tableArn);
    }
}
