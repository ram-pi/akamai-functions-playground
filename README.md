# Akamai Functions Playground

[![EMEA/APJ Clock](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fram-pi%2Fakamai-functions-playground%2Fmain%2F.github%2Ftime-teller-url.json&style=flat-square)](https://1792b14e-3717-462b-ba53-781bd689159a.fwf.app)
[![Wasm Explained](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fram-pi%2Fakamai-functions-playground%2Fmain%2F.github%2Fwasm-explained-url.json&style=flat-square)](https://58372415-f6e8-4cf3-9078-48c93c56d7aa.fwf.app)

A collection of templates and examples for building WebAssembly functions with [Fermyon Spin](https://developer.fermyon.com/spin).

## Templates

Templates are used with `spin templates install` and `spin new` to scaffold new projects.

| Template | Description |
|---|---|
| [`templates/http-rust-basic-auth`](templates/http-rust-basic-auth/README.md) | HTTP handler in Rust with HTTP Basic Authentication |

## Applications

| App | Trigger | Description |
|---|---|---|
| [`time-teller/`](time-teller/) | HTTP | Chess-clock-style world clock rendering 9 timezones as animated SVG analog clocks; tracks visitor count via KV store |
| [`wasm-explained-static-content/`](wasm-explained-static-content/) | HTTP | Static webpage explaining how WebAssembly works, served via `spin-fileserver` |

## Wasm Components

Reusable WebAssembly components under `components/` exposing typed [WIT](https://component-model.bytecodealliance.org/design/wit.html) interfaces. Built with `cargo-component` targeting `wasm32-wasip2` and published to GHCR as OCI artifacts.

| Component | WIT interface | Description |
|---|---|---|
| [`components/time-zone-clock/`](components/time-zone-clock/) | `ram-pi:time-zone-clock/timezones` | Exports `get-timezones()` — timezone name, IANA identifier, and accent color for 9 world clocks; composed into `time_teller_composed.wasm` at build time |

### How composition works

```
components/time-zone-clock/   →  time_zone_clock.wasm  ──┐
                                                           ├─ wasm-tools compose ──► time_teller_composed.wasm
time-teller/                  →  time_teller.wasm       ──┘
```

`time-teller` declares a WIT import (`time-teller/wit/world.wit`) and calls `get_timezones()` at runtime. `wasm-tools compose` links the two binaries so the import is satisfied before Spin loads the app.

### Using the time-zone-clock component from GHCR

The `time-zone-clock` component is published to GHCR on every push to `main` that touches its source:

```bash
oras pull ghcr.io/ram-pi/time-zone-clock:latest
wasm-tools validate time_zone_clock.wasm
```

### Local build

```bash
rustup target add wasm32-wasip2
cargo install cargo-component --locked
cargo install wasm-tools --locked

# Build the component
cd components/time-zone-clock && cargo component build --target wasm32-wasip2 --release

# Build + compose time-teller
cd time-teller && spin build
```

## Useful Commands

### Akamai Functions (aka)

| Command | Description |
|---|---|
| `spin aka apps list --account-id <id>` | List all deployed apps |
| `spin aka app status --app-name time-teller` | Show app status and URL |
| `spin aka app delete --app-name time-teller --account-id <id> --no-confirm` | Delete the app |


## CI/CD

Two workflows run on changes to the relevant paths:

| Workflow | Trigger paths | Description | Required secrets |
|---|---|---|---|
| [`publish-time-zone-clock.yml`](.github/workflows/publish-time-zone-clock.yml) | `components/time-zone-clock/**`, `v*` tags | Builds `time-zone-clock` and pushes to `ghcr.io/ram-pi/time-zone-clock` as an OCI artifact | _(uses `GITHUB_TOKEN`)_ |
| [`deploy-time-teller.yml`](.github/workflows/deploy-time-teller.yml) | `time-teller/**`, `components/time-zone-clock/**` | Builds and composes `time-teller`, then deploys to Akamai Functions via `spin aka deploy` | `AKAMAI_FUNCTIONS_TOKEN`, `AKAMAI_FUNCTIONS_ACCOUNT_ID` |
| [`delete-time-teller.yml`](.github/workflows/delete-time-teller.yml) | manual | Deletes `time-teller` from Akamai Functions | `AKAMAI_FUNCTIONS_TOKEN`, `AKAMAI_FUNCTIONS_ACCOUNT_ID` |
| [`deploy-wasm-explained.yml`](.github/workflows/deploy-wasm-explained.yml) | `wasm-explained-static-content/**` | Deploys the static WebAssembly explainer page to Akamai Functions | `AKAMAI_FUNCTIONS_TOKEN`, `AKAMAI_FUNCTIONS_ACCOUNT_ID` |
| [`delete-wasm-explained.yml`](.github/workflows/delete-wasm-explained.yml) | manual | Deletes `wasm-explained-static-content` from Akamai Functions | `AKAMAI_FUNCTIONS_TOKEN`, `AKAMAI_FUNCTIONS_ACCOUNT_ID` |
