# Test aurl-rust with local auth server and resource server

## Check headers

```bash
target/debug/aurl-rust -p aurl-dev-password http://localhost:$RESOURCE_SERVER_PORT/headers
```

## Redirects

Follow redirect

```bash
target/debug/aurl-rust -p aurl-dev-password http://localhost:$RESOURCE_SERVER_PORT/redirect/1
```

Error when redirect occurs over 5 times.

```bash
target/debug/aurl-rust -p aurl-dev-password http://localhost:$RESOURCE_SERVER_PORT/redirect/6
```

Ignore redirect to different origin.

```bash
target/debug/aurl-rust -p aurl-dev-password http://localhost:$RESOURCE_SERVER_PORT/redirect-to?url=http://example.com
```