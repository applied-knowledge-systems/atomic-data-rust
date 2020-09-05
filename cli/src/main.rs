use atomic_lib::errors::AtomicResult;
use atomic_lib::mapping::Mapping;
use atomic_lib::serialize;
use atomic_lib::Storelike;

use atomic_lib::Db;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand, crate_version};
use colored::*;
use dirs::home_dir;
use serialize::serialize_atoms_to_ad3;
use std::path::PathBuf;

mod delta;
mod new;
mod path;

#[allow(dead_code)]
pub struct Context<'a> {
    store: Db,
    mapping: Mapping,
    matches: ArgMatches<'a>,
    config_folder: PathBuf,
    user_store_path: PathBuf,
    user_mapping_path: PathBuf,
}

fn main() {
    let matches = App::new("atomic")
        .version(crate_version!())
        .author("Joep Meindertsma <joep@ontola.io>")
        .about("Create, share, fetch and model linked atomic data!")
        .after_help("Visit https://github.com/joepio/atomic for more info")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("new").about("Create a Resource")
            .arg(
                Arg::with_name("class")
                    .help("The URL or shortname of the Class that should be created")
                    .required(true),
            )
        )
        .subcommand(
            SubCommand::with_name("get")
                    .about("Traverses a Path and prints the resulting Resource or Value.",
                    )
                    .after_help("\
                    Traverses a Path and prints the resulting Resource or Value. \
                    Examples: \natomic get \"class description\"\natomic get \"https://example.com\"\n\
                    Visit https://docs.atomicdata.dev/core/paths.html for more info about paths. \
                    ")
                .arg(Arg::with_name("path")
                    .help("\
                    The subject URL, shortname or path to be fetched. \
                    Use quotes for paths. \
                    You can use Bookmarks instead of a full subject URL. \
                    ",
                    )
                    .required(true)
                )
                .arg(Arg::with_name("as")
                    .long("as")
                    .help("Serialization option (pretty=default, json, ad3)")
                    .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("tpf")
                    .about("Finds Atoms using Triple Pattern Fragments",
                    )
                    .after_help("\
                    Filter the store by <subject> <property> and <value>. \
                    Use a dot to indicate that you don't need to filter. \
                    ")
                .arg(Arg::with_name("subject")
                    .help("The subject URL or bookmark to be filtered by. Use a dot '.' to indicate 'any'.")
                    .required(true)
                )
                .arg(Arg::with_name("property")
                    .help("The property URL or bookmark to be filtered by. Use a dot '.' to indicate 'any'.")
                    .required(true)
                )
                .arg(Arg::with_name("value")
                    .help("The value URL or bookmark to be filtered by. Use a dot '.' to indicate 'any'.")
                    .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("delta")
                    .about("Update the store using an single Delta",
                    )
                    .after_help("\
                    Use a dot to indicate that you don't need to filter. \
                    ")
                .arg(Arg::with_name("method")
                    .help("Method URL or bookmark, describes how the resource will be changed. Only suppports Insert at the time")
                    .required(true)
                )
                .arg(Arg::with_name("subject")
                    .help("Subject URL or bookmark of the thing to be changed")
                    .required(true)
                )
                .arg(Arg::with_name("property")
                    .help("Property URL or bookmark of the thing that needs to be updated")
                    .required(true)
                )
                .arg(Arg::with_name("value")
                    .help("The new Value serialized as a a string")
                    .required(true)
                )
        )
        .subcommand(SubCommand::with_name("list").about("List all bookmarks"))
        .subcommand(SubCommand::with_name("populate").about("Adds the default Atoms to the store"))
        .subcommand(SubCommand::with_name("validate").about("Validates the store"))
        .get_matches();

    let config_folder = home_dir()
        .expect("Home dir could not be opened. We need this to store data.")
        .join(".config/atomic/");
    let user_mapping_path = config_folder.join("mapping.amp");
    let default_mapping_path = PathBuf::from("../defaults/default_mapping.amp");
    let mut mapping_path = &default_mapping_path;
    if user_mapping_path.exists() {
        mapping_path = &user_mapping_path;
    }
    let mut mapping: Mapping = Mapping::init();
    mapping.read_mapping_from_file(&mapping_path).unwrap();
    let user_store_path = config_folder.join("db");
    let store_path = &user_store_path;
    let store: Db = Db::init(store_path).expect("Failed opening store");

    let mut context = Context {
        mapping,
        store,
        matches,
        config_folder,
        user_store_path: user_store_path.clone(),
        user_mapping_path: user_mapping_path.clone(),
    };

    match context.matches.subcommand_name() {
        Some("new") => {
            new::new(&mut context);
        }
        Some("list") => {
            list(&mut context);
        }
        Some("get") => {
            path::get(&mut context);
        }
        Some("tpf") => {
            tpf(&mut context);
        }
        Some("delta") => {
            delta::delta(&mut context).unwrap();
        }
        Some("populate") => {
            populate(&mut context);
        }
        Some("validate") => {
            validate(&mut context);
        }
        Some(cmd) => println!("{} is not a valid command. Run atomic --help", cmd),
        None => println!("Run atomic --help for available commands"),
    }
}

/// List all bookmarks
fn list(context: &mut Context) {
    let mut string = String::new();
    for (shortname, url) in context.mapping.clone().into_iter() {
        string.push_str(&*format!(
            "{0: <15}{1: <10} \n",
            shortname.blue().bold(),
            url
        ));
    }
    println!("{}", string)
}

/// Prints a resource to the terminal with readble formatting and colors
fn pretty_print_resource(url: &String, store: &dyn Storelike) -> AtomicResult<()> {
    let mut output = String::new();
    let resource = store
        .get_resource_string(url)
        .ok_or(format!("Not found: {}", url))?;
    for (prop_url, val) in resource {
        let prop_shortname = store.property_url_to_shortname(&prop_url).unwrap();
        output.push_str(&*format!(
            "{0: <15}{1: <10} \n",
            prop_shortname.blue().bold(),
            val
        ));
    }
    output.push_str(&*format!("{0: <15}{1: <10} \n", "url".blue().bold(), url));
    println!("{}", output);
    Ok(())
}

/// Triple Pattern Fragment Query
fn tpf(context: &mut Context) {
    let subcommand_matches = context.matches.subcommand_matches("tpf").unwrap();
    let subject = tpf_value(subcommand_matches.value_of("subject").unwrap());
    let property = tpf_value(subcommand_matches.value_of("property").unwrap());
    let value = tpf_value(subcommand_matches.value_of("value").unwrap());
    let found_atoms = context
        .store
        .tpf(subject, property, value)
        .expect("TPF failed");
    let serialized = serialize_atoms_to_ad3(found_atoms);
    println!("{}", serialized.unwrap())
}

fn tpf_value(string: &str) -> Option<String> {
    if string == "." {
        return None;
    } else {
        return Some(string.into());
    }
}

/// Adds the default store to the store
fn populate(context: &mut Context) {
    let ad3 = include_str!("../../defaults/default_store.ad3");
    context
        .store
        .parse_ad3(&String::from(ad3))
        .expect("Error when parsing store");
    println!("Succesfully added default Atoms to the store. Run `atomic-cli tpf . . .` to list them all!");
}

/// Validates the store
fn validate(context: &mut Context) {
    context.store.validate_store().expect("Invalid store");
    println!("Store is valid!");
}
