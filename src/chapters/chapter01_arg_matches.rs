pub fn run() {
    use clap::{App, ArgMatches};

    let matches: ArgMatches = App::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg("-c, --config=[FILE]  'Sets a custom config file'")
        .arg("<INPUT>              'Sets the input file to use'")
        .arg("-v...                'Sets the level of verbosity'")
        .subcommand(App::new("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg("-d, --debug 'Print debug information'"))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let config = matches.value_of("config").unwrap();
    print!("XXX: {} {}", input, config);
}