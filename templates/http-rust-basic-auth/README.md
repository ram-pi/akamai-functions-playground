# http-rust-basic-auth

A Spin HTTP component template written in Rust that protects an endpoint with [HTTP Basic Authentication](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication#basic_authentication_scheme).

Credentials are stored as [Spin variables](https://developer.fermyon.com/spin/v3/variables) and never hard-coded, making them easy to inject at runtime or at deploy time.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) with the `wasm32-wasip1` target:
  ```sh
  rustup target add wasm32-wasip1
  ```
- [Spin CLI](https://developer.fermyon.com/spin/v3/install) v3 or later

## Install the template

```sh
spin templates install --git https://github.com/ram-pi/akamai-functions-playground --update
```

## Scaffold a new project

```sh
spin new -t http-rust-basic-auth my-app
```

Spin will prompt for:

| Prompt | Description | Default |
|---|---|---|
| Description | Short description of the project | *(empty)* |
| HTTP path | Route the component is mounted on | `/...` |

## Build and run locally

```sh
cd my-app
SPIN_VARIABLE_PASSWORD=secret spin up --build
```

`username` defaults to `admin`. Override it the same way:

```sh
SPIN_VARIABLE_USERNAME=alice SPIN_VARIABLE_PASSWORD=secret spin up --build
```

## Test

```sh
# Should return 401
curl -i http://127.0.0.1:3000/

# Should return 200 with "Hello, admin!"
curl -i -u admin:secret http://127.0.0.1:3000/
```

## Configuration

Credentials are exposed as Spin variables in `spin.toml`:

```toml
[variables]
username = { default = "admin" }
password = { required = true, secret = true }
```

You can set them via:

- **Environment variables** (local): `SPIN_VARIABLE_PASSWORD=secret spin up`
- **`.env` file** (local): create a `.env` file next to `spin.toml`:
  ```env
  SPIN_VARIABLE_USERNAME=alice
  SPIN_VARIABLE_PASSWORD=secret
  ```
- **Deploy-time config** (Fermyon Cloud / Akamai): pass variables through your platform's secret store or deploy flags.

## How it works

The handler reads the `Authorization: Basic <base64(username:password)>` header on every request, decodes the credentials, and compares them against the configured variables. On mismatch it returns `401 Unauthorized` with a `WWW-Authenticate` challenge header.
