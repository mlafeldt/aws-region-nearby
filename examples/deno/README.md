Build and deploy the example to Deno Deploy:

```console
export DENO_DEPLOY_TOKEN=...

make deploy
```

Afterward, configure the environment variables `REPLICA_REGIONS`, `TABLE_NAME`, `AWS_ACCESS_KEY_ID`, and `AWS_SECRET_ACCESS_KEY` in the Deno Deploy console.

You can also start a local server for development:

```console
export DENO_REGION=europe-west3
export REPLICA_REGIONS=eu-central-1,us-east-1,ap-northeast-1
export TABLE_NAME=region-nearby-TableCD117FA1-3J53CKB5QHY2
export AWS_ACCESS_KEY_ID=...
export AWS_SECRET_ACCESS_KEY=...

make serve
```
