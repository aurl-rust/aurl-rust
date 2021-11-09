aurl-rust
====

## Description

*aurl-rust* is yet another implementation of [aurl](https://github.com/classmethod/aurl)

## Profile

|parameter    | description | example |
|-------------|-------------|---------|
|`[PROFILE]`  | Unique profile name |`[auth0]`|
|`default_content_type` | Explicit Response Content Type | `application/json` |
|`grant_type` | Specify Authorization grant flow. Allow grant flow are below. | `authorization_code` |
|`client_id`  | Specify ClientId for Auth Server | `5D0AD236-796A-4098-8220-D04D8920F0CA` |
|`client_secret` | Specify ClientSecret for Auth Server | `5C0927EC-C5B8-4237-A99A-EB71D6F41410`
|`auth_server_token_endpoint`| Specify Token Endpoint for Auth Server. See [RFC6749#section-3.2](https://datatracker.ietf.org/doc/html/rfc6749#section-3.2) | `https://example.auth0.com/oauth/token`
|`auth_server_auth_endpoint` | Specify Authorization Endpoint. See [RFC6749#section-3.1](https://datatracker.ietf.org/doc/html/rfc6749#section-3.1) | `https://example.auth0.com/authorize`
|`scopes`     | Specify scope for AccessToken. See [RFC6749#section-3.3](https://datatracker.ietf.org/doc/html/rfc6749#section-3.3) |`openid profile`
|`redirect` | Specify URL for getting Authorizatoin Code. We recommend that you specify `localhost`. You MUST setting explicit redirect URL in Authorization Server Settings. | `http://localhost:8080/callback` |
|`default_auth_header_template` | When you get AccessToken, aurl set AccessToken in specify custom header. According to the following format | `x-hogehoge=hoge $token`

### example

```properties
[auth0]
default_content_type = application/json
grant_type = authorization_code
client_id = XXXXXXXXXXXXXX
client_secret = XXXXXXXXXXXXXX
auth_server_token_endpoint = https://example.auth0.com/oauth/token
auth_server_auth_endpoint = https://example.auth0.com/authorize
scopes = openid profile
redirect = http://localhost:8080/callback
default_auth_header_template=x-hogehoge=hoge $token
```
