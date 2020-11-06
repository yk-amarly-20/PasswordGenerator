extern crate pwd_generator;
use pwd_generator::generator;
use pwd_generator::parser;

fn main() {
    let args = parser::parse_args();
    generator::generate_pwd(&args);
}
