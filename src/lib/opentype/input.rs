use std::io;

pub fn read_be_u32(stream: &mut io::File, count: uint)
    -> Result<Vec<u32>, io::IoError> {

    let mut result: Vec<u32> = Vec::new();

    for _ in range(0, count) {
        match stream.read_be_u32() {
            Ok(value) => result.push(value),
            Err(error) => return Err(error)
        }
    }

    Ok(result)
}
