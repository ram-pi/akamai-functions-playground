mod bindings;
use bindings::ram_pi::time_zone_clock::timezones::get_timezones;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

#[http_component]
fn handle_time_teller(req: Request) -> anyhow::Result<impl IntoResponse> {
    let path = req.path();
    let views = if path == "/" || path.is_empty() {
        increment_views()?
    } else {
        get_views()?
    };
    let html = build_html(views);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(html)
        .build())
}

fn increment_views() -> anyhow::Result<u64> {
    let store = Store::open_default()?;
    let views: u64 = match store.get("views")? {
        Some(bytes) => String::from_utf8(bytes)?.parse().unwrap_or(0),
        None => 0,
    };
    let new_views = views + 1;
    store.set("views", new_views.to_string().as_bytes())?;
    Ok(new_views)
}

fn get_views() -> anyhow::Result<u64> {
    let store = Store::open_default()?;
    Ok(match store.get("views")? {
        Some(bytes) => String::from_utf8(bytes)?.parse().unwrap_or(0),
        None => 0,
    })
}

fn build_html(views: u64) -> String {
    let timezones = get_timezones();
    let zones_js: String = timezones
        .iter()
        .map(|tz| format!(
            r##"  {{ name: "{}", iana: "{}", color: "{}" }}"##,
            tz.name, tz.iana, tz.color
        ))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>EMEA/APJ Clock</title>
  <link rel="icon" type="image/svg+xml" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='48' fill='%231a1a1a' stroke='%23888' stroke-width='4'/><line x1='50' y1='6' x2='50' y2='18' stroke='%23ccc' stroke-width='4' stroke-linecap='round'/><line x1='94' y1='50' x2='82' y2='50' stroke='%23888' stroke-width='3' stroke-linecap='round'/><line x1='50' y1='94' x2='50' y2='82' stroke='%23888' stroke-width='3' stroke-linecap='round'/><line x1='6' y1='50' x2='18' y2='50' stroke='%23888' stroke-width='3' stroke-linecap='round'/><line x1='50' y1='50' x2='28' y2='37' stroke='white' stroke-width='6' stroke-linecap='round'/><line x1='50' y1='50' x2='76' y2='35' stroke='%23ccc' stroke-width='4' stroke-linecap='round'/><circle cx='50' cy='50' r='5' fill='%23888'/></svg>">
  <style>
    *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      background: #0d0d0d;
      color: #f0f0f0;
      font-family: 'Segoe UI', system-ui, sans-serif;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 2rem 1rem;
    }}
    h1 {{
      font-size: 1.8rem;
      letter-spacing: 0.15em;
      margin-bottom: 2rem;
      text-transform: uppercase;
      color: #ccc;
    }}
    .grid {{
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 1.5rem;
      max-width: 960px;
      width: 100%;
    }}
    .panel {{
      background: #1a1a1a;
      border-radius: 12px;
      padding: 1.2rem;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 0.75rem;
      border: 1px solid #2a2a2a;
    }}
    .panel h2 {{
      font-size: 1rem;
      letter-spacing: 0.1em;
      text-transform: uppercase;
    }}
    .clock-face {{
      width: 140px;
      height: 140px;
    }}
    .iana {{
      font-size: 0.7rem;
      letter-spacing: 0.05em;
      color: #666;
      margin-top: -0.4rem;
    }}
    .digital {{
      font-size: 1.1rem;
      font-variant-numeric: tabular-nums;
      letter-spacing: 0.05em;
      color: #aaa;
    }}
    .views-counter {{
      margin-top: 2.5rem;
      font-size: 0.85rem;
      color: #555;
      letter-spacing: 0.08em;
    }}
    .views-counter span {{
      color: #888;
      font-variant-numeric: tabular-nums;
    }}
  </style>
</head>
<body>
  <h1>EMEA/APJ Clock</h1>
  <div class="grid" id="grid"></div>
  <p class="views-counter">VISITORS: <span>{views}</span></p>

  <script>
    const ZONES = [
{zones_js}
    ];

    const grid = document.getElementById('grid');

    ZONES.forEach((zone, i) => {{
      const panel = document.createElement('div');
      panel.className = 'panel';
      panel.innerHTML = `
        <h2 style="color: ${{zone.color}}">${{zone.name}}</h2>
        <div class="iana">${{zone.iana}}</div>
        <svg class="clock-face" viewBox="0 0 100 100" id="svg-${{i}}">
          <circle cx="50" cy="50" r="48" fill="#111111" stroke="${{zone.color}}" stroke-width="2"/>
          ${{buildTicks(zone.color)}}
          <line id="hour-${{i}}"   x1="50" y1="50" x2="50" y2="25" stroke="white"         stroke-width="3" stroke-linecap="round"/>
          <line id="minute-${{i}}" x1="50" y1="50" x2="50" y2="18" stroke="#cccccc"        stroke-width="2" stroke-linecap="round"/>
          <line id="second-${{i}}" x1="50" y1="55" x2="50" y2="14" stroke="${{zone.color}}" stroke-width="1" stroke-linecap="round"/>
          <circle cx="50" cy="50" r="2.5" fill="${{zone.color}}"/>
        </svg>
        <div class="digital" id="digital-${{i}}">--:--:--</div>
      `;
      grid.appendChild(panel);
    }});

    function buildTicks(color) {{
      let ticks = '';
      for (let t = 0; t < 60; t++) {{
        const angle = t * 6;
        const isHour = t % 5 === 0;
        const r1 = isHour ? 40 : 44;
        const r2 = 48;
        const rad = (angle - 90) * Math.PI / 180;
        const x1 = 50 + r1 * Math.cos(rad);
        const y1 = 50 + r1 * Math.sin(rad);
        const x2 = 50 + r2 * Math.cos(rad);
        const y2 = 50 + r2 * Math.sin(rad);
        const w = isHour ? 1.5 : 0.7;
        ticks += `<line x1="${{x1.toFixed(2)}}" y1="${{y1.toFixed(2)}}" x2="${{x2.toFixed(2)}}" y2="${{y2.toFixed(2)}}" stroke="${{isHour ? color : "#444444"}}" stroke-width="${{w}}"/>`;
      }}
      return ticks;
    }}

    function rotateLine(el, deg) {{
      el.setAttribute('transform', `rotate(${{deg}}, 50, 50)`);
    }}

    function tick() {{
      ZONES.forEach((zone, i) => {{
        const fmt = new Intl.DateTimeFormat('en-GB', {{
          timeZone: zone.iana,
          hour: '2-digit', minute: '2-digit', second: '2-digit',
          hour12: false
        }});
        const parts = Object.fromEntries(fmt.formatToParts(new Date()).map(p => [p.type, parseInt(p.value, 10)]));
        const h = parts.hour % 12;
        const m = parts.minute;
        const s = parts.second;

        const hDeg = h * 30 + m * 0.5;
        const mDeg = m * 6 + s * 0.1;
        const sDeg = s * 6;

        rotateLine(document.getElementById(`hour-${{i}}`),   hDeg);
        rotateLine(document.getElementById(`minute-${{i}}`), mDeg);
        rotateLine(document.getElementById(`second-${{i}}`), sDeg);

        const pad = n => String(n).padStart(2, '0');
        document.getElementById(`digital-${{i}}`).textContent =
          `${{pad(parts.hour)}}:${{pad(m)}}:${{pad(s)}}`;
      }});
      requestAnimationFrame(tick);
    }}

    tick();
  </script>
</body>
</html>
"##, zones_js = zones_js, views = views)
}
