aurl-rust
====

## Description

*aurl-rust* is yet another implementation of [aurl](https://github.com/classmethod/aurl)

## Install

Get the binary from [Releases](https://github.com/cm-kazup0n/aurl-rust/releases).

or 

```
$ cargo install --git https://github.com/cm-kazup0n/aurl-rust.git --locked
```

installing with `--locked` is recommended.

## Usage

```
aurl-rust 

USAGE:
    aurl-rust [OPTIONS] <URL>

ARGS:
    <URL>    

OPTIONS:
        --auth-header-template <AUTH_HEADER_TEMPLATE>
            [default: ]

    -d, --data <DATA>
            

    -h, --help
            Print help information

    -H, --header <HEADER>...
            

        --output <OUTPUT>
            Output Option (case insensitive). curl: Output curl command snippet. none: Call URL with
            Got AccessToken [default: none]

    -p, --profile <PROFILE>
            [default: default]

        --timeout <TIMEOUT>
            [default: 30]

    -v, --verbose
            A level of verbosity, and can be used multiple times

    -X, --request <REQUEST>
            [default: GET]
```

## Profile

|parameter    | description | required | example |
|-------------|-------------|---------|---------|
|`[PROFILE]`  | Unique profile name | required |`[auth0]`|
|`default_content_type` | Explicit Response Content Type | not required, but recommended to set | `application/json` |
|`grant_type` | Specify Authorization grant flow. Allow grant flow are below. | required | `authorization_code` |
|`client_id`  | Specify ClientId for Auth Server | required | `5D0AD236-796A-4098-8220-D04D8920F0CA` |
|`client_secret` | Specify ClientSecret for Auth Server | required | `5C0927EC-C5B8-4237-A99A-EB71D6F41410`
|`auth_server_token_endpoint`| Specify Token Endpoint for Auth Server. See [RFC6749#section-3.2](https://datatracker.ietf.org/doc/html/rfc6749#section-3.2) | required | `https://example.auth0.com/oauth/token`
|`auth_server_auth_endpoint` | Specify Authorization Endpoint. See [RFC6749#section-3.1](https://datatracker.ietf.org/doc/html/rfc6749#section-3.1) | required | `https://example.auth0.com/authorize`
|`scopes`     | Specify scope for AccessToken. See [RFC6749#section-3.3](https://datatracker.ietf.org/doc/html/rfc6749#section-3.3) | required | `openid profile`
|`redirect` | Specify URL for getting Authorizatoin Code. We recommend that you specify `localhost`. You MUST setting explicit redirect URL in Authorization Server Settings. |  required |  `http://localhost:8080/callback` |
|`default_auth_header_template` |  When you get AccessToken, aurl set AccessToken in specify custom header. According to the following format. If you don't set, set AccessToken in `authorization` header as `Bearer` Token. | option | `x-hogehoge=hoge $token`

### example (set default_auth_header_template)

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

### example (AccessToken as Bearer Token)

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
```
