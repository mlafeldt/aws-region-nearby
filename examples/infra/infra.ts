import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import * as ddb from "aws-cdk-lib/aws-dynamodb";
import * as iam from "aws-cdk-lib/aws-iam";
import * as ssm from "aws-cdk-lib/aws-ssm";
import { Construct } from "constructs";

interface RegionNearbyStackProps extends cdk.StackProps {
  replicationRegions: string[];
}

class RegionNearbyStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: RegionNearbyStackProps) {
    super(scope, id, props);

    const table = new ddb.Table(this, "Table", {
      partitionKey: { name: "key", type: ddb.AttributeType.STRING },
      replicationRegions: props.replicationRegions,
      billingMode: ddb.BillingMode.PAY_PER_REQUEST,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    const user = new iam.User(this, "User");
    table.grantReadWriteData(user);

    const accessKey = new iam.CfnAccessKey(this, "AccessKey", {
      userName: user.userName,
    });

    const accessKeyParam = new ssm.StringParameter(this, "AccessKeyParam", {
      parameterName: `/${id}/aws-access-key`,
      stringValue: accessKey.ref,
    });
    const secretKeyParam = new ssm.StringParameter(this, "SecretKeyParam", {
      parameterName: `/${id}/aws-secret-key`,
      stringValue: accessKey.attrSecretAccessKey,
    });

    new cdk.CfnOutput(this, "TableName", { value: table.tableName });
    new cdk.CfnOutput(this, "ReplicaRegions", {
      value: [cdk.Stack.of(this).region, ...props.replicationRegions].join(","),
    });

    new cdk.CfnOutput(this, "AccessKeyParamName", {
      value: accessKeyParam.parameterName,
    });
    new cdk.CfnOutput(this, "SecretKeyParamName", {
      value: secretKeyParam.parameterName,
    });
  }
}

const app = new cdk.App();
new RegionNearbyStack(app, "region-nearby", {
  replicationRegions: ["us-east-1", "ap-northeast-1"],
  env: { region: "eu-central-1" },
});
