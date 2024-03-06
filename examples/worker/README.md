Build and deploy the example to Cloudflare Workers:

```console
export CLOUDFLARE_ACCOUNT_ID=...
export CLOUDFLARE_API_TOKEN=...

wrangler publish
```

Afterward, configure the environment variables `REPLICA_REGIONS`, `TABLE_NAME`, `AWS_ACCESS_KEY_ID`, and `AWS_SECRET_ACCESS_KEY` via `wrangler secret put`.

You can also start a local server for development:

```console
wrangler dev --remote
```
