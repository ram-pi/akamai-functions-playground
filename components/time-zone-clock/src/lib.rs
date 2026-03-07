wit_bindgen::generate!({
    world: "time-zone-clock",
});
use exports::ram_pi::time_zone_clock::timezones::{Guest, TimezoneInfo};

struct Component;

impl Guest for Component {
    fn get_timezones() -> Vec<TimezoneInfo> {
        vec![
            TimezoneInfo { name: "Italy".into(),    iana: "Europe/Rome".into(),       color: "#e63946".into() },
            TimezoneInfo { name: "Ireland".into(),  iana: "Europe/Dublin".into(),     color: "#2a9d8f".into() },
            TimezoneInfo { name: "Spain".into(),    iana: "Europe/Madrid".into(),     color: "#e9c46a".into() },
            TimezoneInfo { name: "UK".into(),       iana: "Europe/London".into(),     color: "#457b9d".into() },
            TimezoneInfo { name: "Malaysia".into(), iana: "Asia/Kuala_Lumpur".into(), color: "#f4a261".into() },
            TimezoneInfo { name: "Japan".into(),    iana: "Asia/Tokyo".into(),        color: "#e76f51".into() },
            TimezoneInfo { name: "India".into(),    iana: "Asia/Kolkata".into(),      color: "#a8dadc".into() },
            TimezoneInfo { name: "Taiwan".into(),   iana: "Asia/Taipei".into(),       color: "#6a4c93".into() },
            TimezoneInfo { name: "China".into(),    iana: "Asia/Shanghai".into(),     color: "#80b918".into() },
        ]
    }
}

export!(Component);
