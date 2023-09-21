use clap::{crate_version, App, Arg};

pub fn arc_app() -> App<'static, 'static> {
    App::new("spaceo")
        .version(&crate_version!()[..])
        .about("Simple Directory Information")
        .arg(
            Arg::with_name("nums")
                .help("Number of biggest files to display")
                .short("n")
                .takes_value(true)
        )
}
