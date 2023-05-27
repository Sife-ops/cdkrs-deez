import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class CdkrsStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'CdkrsQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });

    // new cdk.aws_s3.Bucket(this, "MyFirstBucket", {
    //   versioned: true,
    // });

    new RustFunction(this, "function-package-name", {
      manifestPath: "functions/hw/Cargo.toml",
      // layers: [
      //   extensionLayer
      // ],
    });
  }
}
