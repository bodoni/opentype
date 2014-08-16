#![crate_name = "support"]
#![crate_type = "rlib"]

#![feature(globs, macro_rules)]

static FIXTURE_VARIABLE: &'static str = "FIXTURE_PATH";

pub fn find_fixture(name: &str) -> Option<Path> {
    match read_environment(FIXTURE_VARIABLE) {
        Some(value) => Some(Path::new(value).join(name)),
        None => None
    }
}

fn read_environment(name: &str) -> Option<String> {
    for &(ref key, ref value) in std::os::env().iter() {
        if key.as_slice() == name {
            return Some(value.clone());
        }
    }
    None
}

#[macro_export]
macro_rules! open_fixture(
    ($name:expr) => (
        ::std::io::File::open(&find_fixture($name).unwrap()).unwrap()
    )
)
