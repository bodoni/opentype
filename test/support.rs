use std::os;

static FIXTURE_VARIABLE: &'static str = "FIXTURE_PATH";

pub fn find_fixture(name: &str) -> Option<Path> {
    match read_environment(FIXTURE_VARIABLE) {
        Some(value) => Some(Path::new(value).join(name)),
        None => None
    }
}

fn read_environment(name: &str) -> Option<String> {
    for &(ref key, ref value) in os::env().iter() {
        if key.as_slice() == name {
            return Some(value.clone());
        }
    }
    None
}
