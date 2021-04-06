use clap::{App, Arg};
use passgen::password::{generator::PasswordGenerator, password::PASSWORD_MAX_LENGTH, symbol};

fn main() {
    let about = format!(
        "This command line tool generate secure random password. \n\
        By default, password consists of lower alphabet and upper alphabet \n\
        and numeric and marks such as \"{}\".",
        symbol::DEFAULT_MARK
    );
    let app = App::new("passgen")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(&about[..])
        .arg(Arg::with_name("no_lower").help("no lower character").short("l").long("lower"))
        .arg(Arg::with_name("no_upper").help("no upper character").short("u").long("upper"))
        .arg(Arg::with_name("no_numeric").help("no numeric").short("n").long("numeric"))
        .arg(Arg::with_name("no_mark").help("no default mark").short("m").long("mark"))
        .arg(
            Arg::with_name("addition")
                .help("custom addition")
                .short("a")
                .long("addition")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("length")
                .help("password length(default: 20)")
                .short("e")
                .long("length")
                .takes_value(true),
        );
    let arg_matches = app.get_matches();

    let mut generator = PasswordGenerator::default();
    generator.use_lower = !arg_matches.is_present("no_lower");
    generator.use_upper = !arg_matches.is_present("no_upper");
    generator.use_numeric = !arg_matches.is_present("no_numeric");
    if arg_matches.is_present("no_mark") {
        generator.addition.clear();
    }
    if let Some(add) = arg_matches.value_of("addition") {
        for c in add.chars() {
            generator.addition.insert(c);
        }
    }
    if let Some(len) = arg_matches.value_of("length") {
        generator.len = len
            .parse()
            .expect(&format!("length must be integer smaller than {}", PASSWORD_MAX_LENGTH));
    }
    println!("{:?}", generator.generate().expect("invalid setting"));
}
