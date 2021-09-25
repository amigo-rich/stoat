use clap::{App, Arg, SubCommand};
use std::path::Path;
use stoat::{operation::Operation, run};

fn main() {
    let matches = App::new("stoat")
        .version("0.1")
        .author("Richard Bradshaw")
        .about("An image indexer")
        .subcommand(
            SubCommand::with_name("category").arg(
                Arg::with_name("add")
                    .short("a")
                    .long("add")
                    .required(false)
                    .takes_value(true),
            ),
        )
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
        .subcommand(
            SubCommand::with_name("location")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("latitude")
                        .long("latitude")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("longitude")
                        .long("longitude")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("category") {
        if let Some(add) = matches.value_of("add") {
            run(Operation::AddCategory(add)).unwrap();
        }
    } else if let Some(matches) = matches.subcommand_matches("index") {
        let path = Path::new(matches.value_of("path").unwrap());
        run(Operation::Index(path.to_path_buf())).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("list") {
        if let Some(like) = matches.value_of("like") {
            run(Operation::ListLike(like)).unwrap();
        } else {
            run(Operation::List).unwrap();
        }
    } else if let Some(location) = matches.subcommand_matches("location") {
        let name = location.value_of("name").unwrap();
        let latitude = location.value_of("latitude").unwrap();
        let latitude: f32 = latitude.trim().parse().unwrap();
        let longitude = location.value_of("longitude").unwrap();
        let longitude: f32 = longitude.trim().parse().unwrap();
        let operation = Operation::AddLocation(name, latitude, longitude);
        run(operation).unwrap();
    }
}
