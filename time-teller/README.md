# time-teller

A Spin HTTP application that serves a chess-clock-style world clock: a dark-themed 3×3 grid of animated SVG analog clocks, one per timezone, all ticking in real time in the browser.

## Structure

```
time-teller/          ← Spin HTTP app (this directory)
  Cargo.toml
  spin.toml
  src/lib.rs

components/
  time-zone-clock/    ← pure Rust library (no Spin SDK)
    Cargo.toml
    src/lib.rs
```

## Reusable component: `time-zone-clock`

`components/time-zone-clock` is a plain Rust library — no Spin SDK dependency — that owns all timezone metadata. It exports a single struct and a const slice:

```rust
// components/time-zone-clock/src/lib.rs

pub struct TimeZoneInfo {
    pub name: &'static str,   // display name, e.g. "Japan"
    pub iana: &'static str,   // IANA timezone ID, e.g. "Asia/Tokyo"
    pub color: &'static str,  // accent hex color, e.g. "#e76f51"
}

pub const TIMEZONES: &[TimeZoneInfo] = &[
    TimeZoneInfo { name: "Italy",    iana: "Europe/Rome",       color: "#e63946" },
    TimeZoneInfo { name: "Ireland",  iana: "Europe/Dublin",     color: "#2a9d8f" },
    // ...
];
```

Its `Cargo.toml` declares `crate-type = ["lib"]` (not `cdylib`), making it a normal Rust library that any crate in the workspace can depend on at compile time.

## How `time-teller` consumes it

### 1. Path dependency in `Cargo.toml`

```toml
[dependencies]
time-zone-clock = { path = "../components/time-zone-clock" }
```

This is a standard Cargo path dependency. Because `time-zone-clock` is a plain `lib` crate, Cargo compiles it as a regular Rust crate and links it into `time-teller` at build time. No special Spin or Wasm machinery is needed.

### 2. Import and use in `src/lib.rs`

```rust
use time_zone_clock::TIMEZONES;

fn build_html() -> String {
    let zones_js: String = TIMEZONES
        .iter()
        .map(|tz| format!(
            r##"  {{ name: "{}", iana: "{}", color: "{}" }}"##,
            tz.name, tz.iana, tz.color
        ))
        .collect::<Vec<_>>()
        .join(",\n");

    // zones_js is embedded verbatim into the HTML as:
    // const ZONES = [ ... ];
}
```

`TIMEZONES` is iterated at handler invocation time to produce a JavaScript `const ZONES = [...]` array that is injected into the HTML response. No timezone logic lives in Rust — the browser handles local time conversion via `Intl.DateTimeFormat` with the `timeZone` option from each entry's `iana` field.

### Why this split?

| Concern | Where it lives |
|---|---|
| Timezone metadata (names, IANA IDs, colors) | `components/time-zone-clock` |
| HTTP handling, HTML/JS generation | `time-teller` |
| Clock rendering, time conversion | Browser (`Intl.DateTimeFormat`) |

Keeping metadata in a separate, dependency-free crate means it can be reused by other Spin apps or non-Spin tools without pulling in the Spin SDK.

## Build & run

```bash
cd time-teller
spin build --up
```

The app starts on `http://127.0.0.1:3000`.

### Smoke test

```bash
curl -s http://127.0.0.1:3000/ | grep -o 'const ZONES'
# const ZONES
```

## Timezones

| Country | IANA ID | Accent color |
|---|---|---|
| Italy | `Europe/Rome` | `#e63946` |
| Ireland | `Europe/Dublin` | `#2a9d8f` |
| Spain | `Europe/Madrid` | `#e9c46a` |
| UK | `Europe/London` | `#457b9d` |
| Malaysia | `Asia/Kuala_Lumpur` | `#f4a261` |
| Japan | `Asia/Tokyo` | `#e76f51` |
| India | `Asia/Kolkata` | `#a8dadc` |
| Taiwan | `Asia/Taipei` | `#6a4c93` |
| China | `Asia/Shanghai` | `#80b918` |
