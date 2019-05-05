pub mod conversion;
pub mod xor;

use std::fs;

pub fn hamming_distance(in1 : &Vec<u8>, in2: &Vec<u8>) -> u32{
    let xor = xor::xor_vec8(in1, in2);
    xor.iter().map(|x| x.count_ones()).sum()
}


pub fn read_file_base64(filename : String) -> Vec<u8> {
    let input = fs::read_to_string(filename).unwrap();
    let input = input.replace("\n", "");
    base64::decode(&input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate hex;

    #[test]
    fn test_hamming_distance() {
        // Set 1 - Challenge 6
        let input1 = "this is a test".to_string().into_bytes();
        let input2 = "wokka wokka!!!".to_string().into_bytes();

        assert_eq!(hamming_distance(&input1, &input2), 37);
    }
}
