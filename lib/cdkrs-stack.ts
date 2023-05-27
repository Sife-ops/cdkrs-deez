import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";

export class CdkrsStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const api = new apigateway.RestApi(this, "api", {
      // description: "example api gateway",
      // deployOptions: {
      //   stageName: "dev",
      // },
      // 👇 enable CORS
      defaultCorsPreflightOptions: {
        allowHeaders: [
          "Content-Type",
          "X-Amz-Date",
          "Authorization",
          "X-Api-Key",
        ],
        allowMethods: ["OPTIONS", "GET", "POST", "PUT", "PATCH", "DELETE"],
        allowCredentials: true,
        allowOrigins: ["http://localhost:3000"],
      },
      endpointTypes: [apigateway.EndpointType.REGIONAL],
    });

    const hwr = api.root.addResource("hw");

    const hwl = new RustFunction(this, "hw", {
      manifestPath: "functions/hw/Cargo.toml",
      // layers: [
      //   extensionLayer
      // ],
    });

    hwr.addMethod(
      "GET",
      new apigateway.LambdaIntegration(hwl, { proxy: true })
    );
  }
}
