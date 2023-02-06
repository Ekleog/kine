use kine::{
    icu::{self, cal::Iso},
    tz::Utc,
    Calendar, Time,
};

fn main() {
    let now = Time::now();
    let iso_utc = icu::Cal::new(Iso, Utc);
    println!("Hello world! According to a few calendars (in UTC), we currently are:");
    println!(" - {}", iso_utc.write(&now).unwrap());
    // TODO: add an example with other calendars
}
