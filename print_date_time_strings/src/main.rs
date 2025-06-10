use chrono::Local;

// TODO: Make some general format examples
fn time_format_year() {
    let format = "%Y";
    let timestamp = Local::now().format(format);
    println!("Format: Year - %Y\n{}\n\n", timestamp);
}

fn simple_clock() {
    let format = "%-I:%M:%S%p";
    let timestamp = Local::now().format(format).to_string().to_lowercase();
    println!("simple clock via funciton:\n{}\n\n", timestamp);
}

fn am_pm_lc() {
    let format = "%p";
    let timestamp = Local::now().format(format).to_string().to_lowercase();
    println!("am/pm (lowercase) via funciton:\n{}\n\n", timestamp);
}

// fn time_format_month_string_long() {
//     let timestamp = Local::now().format("%Y");
//     println!("Format: Year - %Y\n{}\n\n", timestamp);
// }

fn time_rfc3339() {
    let timestamp = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    println!("RFC 3339\n{}\n\n", timestamp);
}

fn time_rfc3339_with_ms() {
    let timestamp = Local::now().to_rfc3339();
    println!("RFC 3339 (with mc)\n{}\n\n", timestamp);
}

fn time_rfc2822() {
    let timestamp = Local::now().to_rfc2822();
    println!("RFC 2822\n{}\n\n", timestamp);
}

fn time_tokens() {
    // Note: the `%%` is an escape to print
    // the single `%` in the output without
    // converting it. This uses a specific
    // datetime to show where leading
    // characters and spaces come into
    // play
    let timestamp =
        chrono::NaiveDateTime::parse_from_str("2023-01-02 14:05:06.12345", "%Y-%m-%d %H:%M:%S.%f")
            .unwrap();
    let format = r#"
Year: 
%%Y: "%Y" | %%y: "%y"

Month:
%%B: "%B" | %%b "%b" | %%m: "%m" | %%-m: "%-m"

Day of Month:
%%d: "%d" | %%-d: "%-d"

Day of Week:
%%A: "%A" | %%a: "%a" | %%w "%w"

Day of Year:
%%j: "%j" | %%-j "%-j"

Hour:
%%H: "%H" | %%-H: "%-H" | %%I: "%I" | %%-I: "%-I"
(%%H is 0 padded if < 10)
(%%-H is not 0 padded if < 10)

Minute:
%%M: "%M" | %%-M: "%-M"

Second:
%%S: "%S" | %%-S: "%-S"

Microsecond:
%%f: "%f" | %%-f: "%-f"

Sunday Based Week Number:
%%U: "%U" | %%-U: "%-U"

Monday Based Week Number:
%%W: "%W" | %%-W: "%-W"

AM/PM:
%%p: "%p"

Locale DateTime String:
%%c: "%c"

Locale Date String:
%%x: "%x"

Locale Time String:
%%X: "%X"


"#;
    println!("{}\n\n", timestamp.format(format));
}

fn main() {
    am_pm_lc();
    simple_clock();
    time_format_year();
    time_rfc3339();
    time_rfc3339_with_ms();
    time_rfc2822();
    time_tokens();
}

// https://docs.rs/chrono/latest/chrono/index.html
