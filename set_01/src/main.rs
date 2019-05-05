extern crate hex;
extern crate base64;

use std::str;
use std::fs;
mod helpers;

use openssl::symm::{decrypt, Cipher};


fn main() {
    // Challenge 1
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    println!("Challenge 1: {}", helpers::conversion::hex_to_base64(input));

    // Challenge 2
    let input1 = "1c0111001f010100061a024b53535009181c".to_string();
    let input1 = hex::decode(input1).unwrap();

    let input2 = "686974207468652062756c6c277320657965".to_string();
    let input2 = hex::decode(input2).unwrap();

    let result = helpers::xor::xor_vec8(&input1, &input2);
    let result = hex::encode(result);

    println!("Challenge 2: {}", result);

    // Challenge 3
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
    let input = hex::decode(input).unwrap();
    let result = helpers::xor::break_single_byte(&input);
    println!("Challenge 3: {}", result);


    // Challenge 4
    let input = fs::read_to_string("assets/4.txt")
        .expect("Something went wrong reading the input for challenge 4");
    let input = input.split("\n");

    let mut best_score : f64 = 0.0;
    let mut best_result : String = "".to_string();

    for s in input {
        let s = hex::decode(s).unwrap();
        let result = helpers::xor::break_single_byte(&s);
        let score = helpers::xor::compute_score(&result);

        if score > best_score && result.len() > 0 {
            best_score = score;
            best_result = result;
        }
    }

    println!("Challenge 4:{}", best_result);

    // Challenge 5
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
        .to_string().into_bytes();
    let key = "ICE".to_string().into_bytes();

    let result = helpers::xor::xor_vec8(&input, &key);
    let result = hex::encode(result);
    println!("Challenge 5: {}", result);


    // Challenge 6
    let input = helpers::read_file_base64("assets/6.txt".to_string());

    let mut best_keysize = 0;
    let mut best_score : f64 = std::f64::MAX;

    for keysize in 1..50 {
        let mut chunks = input.chunks(keysize);
        let mut in1 = chunks.next().unwrap().to_vec();

        let mut total_dist = 0;
        for _ in 1..30 {
            let in2 = chunks.next().unwrap().to_vec();
            total_dist +=  helpers::hamming_distance(&in1, &in2);
            in1 = in2;
        }

        let score = (total_dist as f64) / (keysize as f64);

        if score < best_score {
            best_score = score;
            best_keysize = keysize;
        }
    }
    let mut key : String = String::new();

    for i in 0..best_keysize {
        let cyphertext : Vec<u8> = input[i..].iter().step_by(best_keysize).map(|&x| x).collect();
        let x : char = helpers::xor::find_key_single_byte(&cyphertext) as char;
        key.push(x)
    }
    println!("Challenge 6: {}", key);


    // Challenge 7
    let input = helpers::read_file_base64("assets/7.txt".to_string());
    let key = b"YELLOW SUBMARINE";

    let cipher = Cipher::aes_128_ecb();
    let plaintext = decrypt(cipher, key, None, &input).unwrap();
    let plaintext = str::from_utf8(&plaintext).unwrap();

    let line = plaintext.split('\n').next().unwrap();
    println!("Challenge 7: {}", line);

    // Challenge 8
    let input = fs::read_to_string("assets/8.txt").unwrap();
    let mut max_repeats = 0;
    let mut max_i = 0;
    let block_size = 16;

    for (i, line) in input.split('\n').enumerate() {
        let line = hex::decode(line).unwrap();
        let mut repeats = 0;

        if line.len() == 0 {
            continue;
        }

        // TODO: better duplication detection
        for block_1 in 0..10 {
            for block_2 in 0..block_1 {
                let block_1 = line.chunks(block_size).nth(block_1);
                let block_2 = line.chunks(block_size).nth(block_2);

                if block_1 == block_2 {
                    repeats += 1;
                }
            }
        }

        if repeats > max_repeats {
            max_repeats = repeats;
            max_i = i;
        }
    }

    println!("Challenge 8: {}", max_i);
}
