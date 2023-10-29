#![allow(
    clippy::needless_return,
    clippy::let_and_return,
    clippy::needless_match,
    dead_code
)]

mod tle {
    pub mod structure;
    pub mod tle_main;
}

mod calculate {
    pub mod calculate_main;
}

mod geolocation {
    pub mod geolocation_main;
}

mod notifier {
    pub mod notifier_main;
}

mod units;

fn main() {
    calculate::calculate_main::calculate_overhead_satellites();
    notifier::notifier_main::notify();
}
