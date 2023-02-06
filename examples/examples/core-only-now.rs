use kine_core::Calendar;

fn main() {
    println!("Hello World! Right now we are:");
    println!(" - According to UTC: {}", kine_core::tz::Utc.now().any());
    println!(
        " - On the system clock: {}",
        kine_core::tz::System.now().any()
    );
    println!(
        " - On kine's internal clock counting: {:?}",
        kine_core::Time::now()
    );
}
