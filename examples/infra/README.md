Deploy a replicated DynamoDB table into your AWS account that can be used by the Deno and Cloudflare Workers examples:

```console
export AWS_REGION=eu-central-1
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...

yarn install
yarn cdk bootstrap
yarn cdk deploy
```

The stack outputs include everything you need to configure the examples:

```
Outputs:
region-nearby.AccessKeyParamName = /region-nearby/aws-access-key
region-nearby.ReplicaRegions = eu-central-1,us-east-1,ap-northeast-1
region-nearby.SecretKeyParamName = /region-nearby/aws-secret-key
region-nearby.TableName = region-nearby-TableCD117FA1-7MBGFF633ATB
```
