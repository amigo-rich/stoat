use clap::{App, Arg, SubCommand};
use std::path::Path;
use stoat::{operation::Operation, run};

fn main() {
    let matches = App::new("stoat")
        .version("0.1")
        .author("Richard Bradshaw")
        .about("An image indexer")
        .subcommand(
            SubCommand::with_name("index").arg(
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("list").arg(
                Arg::with_name("like")
                    .short("l")
                    .long("--like")
                    .required(false)
                    .takes_value(true),
            ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("index") {
        let path = Path::new(matches.value_of("path").unwrap());
        run(Operation::Index(path.to_path_buf())).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("list") {
        if let Some(like) = matches.value_of("like") {
            run(Operation::ListLike(like)).unwrap();
        } else {
            run(Operation::List).unwrap();
        }
    }
}
