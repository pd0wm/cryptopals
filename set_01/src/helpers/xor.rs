use std::str;
use std::collections::HashMap;

pub fn xor_vec8(s1 : &Vec<u8>, s2: &Vec<u8>) -> Vec<u8> {
    s1.iter().zip(s2.iter().cycle()).map(|(&a, &b)| a ^ b).collect()
}

pub fn compute_score(s : &str) -> f64 {
    let s = s.to_string().to_lowercase();

    let freq_map : HashMap<char, f64> = vec![
        ('a', 0.081670), ('b', 0.014920), ('c', 0.027820), ('d', 0.042530),
        ('e', 0.127020), ('f', 0.022280), ('g', 0.020150), ('h', 0.060940),
        ('i', 0.060940), ('j', 0.001530), ('k', 0.007720), ('l', 0.040250),
        ('m', 0.024060), ('n', 0.067490), ('o', 0.075070), ('p', 0.019290),
        ('q', 0.000950), ('r', 0.059870), ('s', 0.063270), ('t', 0.090560),
        ('u', 0.027580), ('v', 0.009780), ('w', 0.023600), ('x', 0.001500),
        ('y', 0.019740), ('z', 0.000740), (' ', 0.130000),
    ].into_iter().collect();

    let mut score : f64 = 0.0;
    for c in s.chars() {
        score += match freq_map.get(&c) {
            Some(x) => *x,
            None => 0.0,
        };
    }

    score
}

pub fn find_key_single_byte(input : &Vec<u8>) -> u8 {
    let mut best_key : u8 = 0;
    let mut best_score : f64 = 0.0;

    // Try to decode with all printable keys
    for i in 0..255 {
        let key : Vec<u8> = [i].to_vec();
        let result = xor_vec8(&input, &key);
        let result = str::from_utf8(&result);

        match result {
            Ok(v) => {
                // Keep track of result with best score
                let score = compute_score(v);
                if score > best_score {
                    best_score = score;
                    best_key = i;
                }
            },
            Err(_e) => (),
        }
    }

    best_key
}

pub fn break_single_byte(input : &Vec<u8>) -> String {
    let best_key = find_key_single_byte(input);

    // Decode input with best key
    let key : Vec<u8> = [best_key].to_vec();
    let result = xor_vec8(&input, &key);

    let result = str::from_utf8(&result);
    match result {
        Ok(v) => v.to_string(),
        Err(_e) => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate hex;

    #[test]
    fn test_single_char_xor() {
        // Set 1 - Challenge 3
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            .to_string();
        let input = hex::decode(input).unwrap();
        let result = break_single_byte(&input);

        let correct = "Cooking MC's like a pound of bacon".to_string();
        assert_eq!(result, correct);
    }

    #[test]
    fn test_xor_vec8() {
        // Set 1 - Challenge 2
        let input1 = "1c0111001f010100061a024b53535009181c".to_string();
        let input1 = hex::decode(input1).unwrap();

        let input2 = "686974207468652062756c6c277320657965".to_string();
        let input2 = hex::decode(input2).unwrap();

        let result = xor_vec8(&input1, &input2);
        let result = hex::encode(result);

        let correct = "746865206b696420646f6e277420706c6179".to_string();

        assert_eq!(result, correct);
    }

    #[test]
    fn test_xor_vec8_repeat() {
        // Set 1 - Challenge 5
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
            .to_string().into_bytes();
        let key = "ICE".to_string().into_bytes();
        let result = hex::encode(xor_vec8(&input, &key));

        let correct = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            .to_string();

        assert_eq!(result, correct);
    }
}
