import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import {
  CorsHttpMethod,
  HttpApi,
  HttpMethod,
} from "@aws-cdk/aws-apigatewayv2-alpha";
import { HttpLambdaIntegration } from "@aws-cdk/aws-apigatewayv2-integrations-alpha";

export class CdkrsStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // https://github.com/bobbyhadz/aws-cdk-http-api-apigateway-v2-example/blob/cdk-v2/lib/cdk-starter-stack.ts
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
    });

    api.addRoutes({
      path: "/hw",
      methods: [HttpMethod.GET],
      integration: new HttpLambdaIntegration("hw-integration", hwl),
    });
  }
}
