import { AttributeType, Billing, ProjectionType, TableV2 } from "aws-cdk-lib/aws-dynamodb";
import { Construct } from "constructs";

export interface DatabaseProps {
    environment: string;
}

export class Database extends TableV2 {
    constructor(scope: Construct, id: string, { environment }: DatabaseProps) {
        super(scope, id, {
            tableName: `config-service-${environment}`,
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
    }
}
