Deploy a replicated DynamoDB table into your AWS account that can be used by the Deno and Cloudflare Workers examples:

```console
export AWS_REGION=eu-central-1
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...
```

```console
bun install
bun cdk bootstrap
bun cdk deploy
```

The stack outputs include everything you need to configure the examples:

```
Outputs:
region-nearby.AccessKeyParamName = /region-nearby/aws-access-key
region-nearby.ReplicaRegions = eu-central-1,us-east-1,ap-northeast-1
region-nearby.SecretKeyParamName = /region-nearby/aws-secret-key
region-nearby.TableName = region-nearby-TableCD117FA1-3J53CKB5QHY2
```

```console
aws ssm get-parameter --name /region-nearby/aws-access-key
aws ssm get-parameter --name /region-nearby/aws-secret-key
```
