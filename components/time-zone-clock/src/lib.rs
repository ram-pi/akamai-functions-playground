pub struct TimeZoneInfo {
    pub name: &'static str,
    pub iana: &'static str,
    pub color: &'static str,
}

pub const TIMEZONES: &[TimeZoneInfo] = &[
    TimeZoneInfo { name: "Italy",    iana: "Europe/Rome",       color: "#e63946" },
    TimeZoneInfo { name: "Ireland",  iana: "Europe/Dublin",     color: "#2a9d8f" },
    TimeZoneInfo { name: "Spain",    iana: "Europe/Madrid",     color: "#e9c46a" },
    TimeZoneInfo { name: "UK",       iana: "Europe/London",     color: "#457b9d" },
    TimeZoneInfo { name: "Malaysia", iana: "Asia/Kuala_Lumpur", color: "#f4a261" },
    TimeZoneInfo { name: "Japan",    iana: "Asia/Tokyo",        color: "#e76f51" },
    TimeZoneInfo { name: "India",    iana: "Asia/Kolkata",      color: "#a8dadc" },
    TimeZoneInfo { name: "Taiwan",   iana: "Asia/Taipei",       color: "#6a4c93" },
    TimeZoneInfo { name: "China",    iana: "Asia/Shanghai",     color: "#80b918" },
];
