# Akamai Functions Playground

A collection of templates and examples for building WebAssembly functions with [Fermyon Spin](https://developer.fermyon.com/spin).

## Templates

Templates are used with `spin templates install` and `spin new` to scaffold new projects.

| Template | Description |
|---|---|
| [`templates/http-rust-basic-auth`](templates/http-rust-basic-auth/README.md) | HTTP handler in Rust with HTTP Basic Authentication |

## Applications

| App | Trigger | Description |
|---|---|---|
| [`time-teller/`](time-teller/) | HTTP | Chess-clock-style world clock rendering 9 timezones as animated SVG analog clocks |

## Components

Reusable pure-Rust libraries consumed as path dependencies by Spin apps.

| Component | Description |
|---|---|
| [`components/time-zone-clock/`](components/time-zone-clock/) | Exports `TIMEZONES` — timezone name, IANA identifier, and accent color metadata used by `time-teller` |
