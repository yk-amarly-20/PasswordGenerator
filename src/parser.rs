use clap::{App, AppSettings, Arg};

use crate::args::ComArgs;

pub fn parse_args() -> ComArgs {
    let app = App::new("parser")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Password Generator")
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::from_usage("-u 'not use upper case'"))
        .arg(Arg::from_usage("-i 'not use integer'"))
        .arg(Arg::from_usage("-s 'not use symbol'"))
        .arg(Arg::from_usage("-l --length [length] 'pwd length'").default_value("8"))
        .arg(Arg::from_usage("-n --num [num] 'num of pwd'").default_value("5"))
        .get_matches();

    ComArgs {
        upper: app.is_present("u"),
        int: app.is_present("i"),
        symbol: app.is_present("s"),
        length: value_t!(app, "length", usize).unwrap(),
        num: value_t!(app, "num", usize).unwrap(),
    }
}
