import {
  CorsHttpMethod,
  HttpApi,
  HttpMethod,
} from "@aws-cdk/aws-apigatewayv2-alpha";
import { HttpLambdaIntegration } from "@aws-cdk/aws-apigatewayv2-integrations-alpha";
import * as cdk from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";

export class CdkrsStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    /**
     * https://github.com/bobbyhadz/aws-cdk-dynamodb-table/blob/cdk-v2/lib/cdk-starter-stack.ts
     */
    const table = new dynamodb.Table(this, id, {
      tableName: "cdkrs-table",
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      partitionKey: { name: "pk", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "sk", type: dynamodb.AttributeType.STRING },
      pointInTimeRecovery: true,
    });

    // table.addLocalSecondaryIndex({
    //   indexName: 'statusIndex',
    //   sortKey: {name: 'status', type: dynamodb.AttributeType.STRING},
    //   projectionType: dynamodb.ProjectionType.ALL,
    // });

    // todo: table.addGlobalSecondaryIndex

    /**
     * example: https://github.com/bobbyhadz/aws-cdk-http-api-apigateway-v2-example/blob/cdk-v2/lib/cdk-starter-stack.ts
     * graduation: https://github.com/aws/aws-cdk/discussions/24038
     */
    const api = new HttpApi(this, "cdkrs-api", {
      corsPreflight: {
        allowHeaders: [
          "Content-Type",
          "X-Amz-Date",
          "Authorization",
          "X-Api-Key",
        ],
        allowMethods: [
          CorsHttpMethod.OPTIONS,
          CorsHttpMethod.GET,
          CorsHttpMethod.POST,
          CorsHttpMethod.PUT,
          CorsHttpMethod.PATCH,
          CorsHttpMethod.DELETE,
        ],
        allowCredentials: true,
        allowOrigins: ["http://localhost:3000"],
      },
    });

    new cdk.CfnOutput(this, "api", { value: api.url || "MISSING" });

    const hwl = new RustFunction(this, "hw", {
      manifestPath: "functions/hw/Cargo.toml",
      // todo: https://github.com/cargo-lambda/cargo-lambda-cdk#rust-extension
      bundling: {
        environment: {
          TABLE_NAME: table.tableName,
        },
      },
    });

    table.grantFullAccess(hwl);

    api.addRoutes({
      path: "/hw",
      methods: [HttpMethod.GET],
      integration: new HttpLambdaIntegration("hw-integration", hwl),
    });
  }
}
