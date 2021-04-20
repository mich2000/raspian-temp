fn main() {
    println!("Hello, world!");
}

#[test]
fn test_out_file() {
    use std::io::Read;

    let mut temp_vector = [0;4];
    let file_pi_temp = std::fs::File::open("foo.txt").unwrap();
    let mut handle = file_pi_temp.take(4);
    handle.read_exact(&mut temp_vector).unwrap();
    // Converts '"450' as a example buffer and takes 
    let string_buffer = std::str::from_utf8(&temp_vector[1..]).unwrap();
    let temp = string_buffer.parse::<u16>().unwrap();
    assert_eq!(45, temp / 10);
}
