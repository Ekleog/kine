use kine::{
    icu::{self, cal::Iso},
    tz::Utc,
    Calendar, Time,
};

fn main() {
    let now = Time::now();
    println!("Hello world! According to a few calendars (in UTC), we currently are:");

    // Simple case: ISO 8601 is supported by the Debug impl
    println!(
        " - ISO 8601: {:?}",
        icu::Cal::new(Iso, Utc).write(&now).unwrap()
    );

    // Hard cases: actually configuring the calendar and formatting
    /* TODO: icu-datetime currently fails to build?
    let options =
        length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short).into();
    let coptic_utc = icu::Cal::new(Coptic, Utc);
    let coptic_formatter = TypedDateTimeFormatter::<Coptic>::try_new_unstable(
        &icu_testdata::unstable(),
        &locale!("en").into(),
        options,
    )
    .unwrap();
    println!(" - Coptic: {}", coptic_formatter.format(coptic_utc.write(&now).unwrap().icu().unwrap());

    let indian_utc = icu::Cal::new(Indian, Utc);
    let indian_formatter = TypedDateTimeFormatter::<Indian>::try_new_unstable(
        &icu_testdata::unstable(),
        &locale!("en").into(),
        options,
    )
    .unwrap();
    println!(" - Indian: {}", indian_formatter.format(indian_utc.write(&now).unwrap().icu()).unwrap());
    */
}
