mod parser;

fn main() {
    let programs = parser::parse_config_file("config.yaml").expect("Config file parse failed");
    println!("{programs:#?}");
}
