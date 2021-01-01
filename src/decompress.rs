use crc32fast::Hasher;
use rand::Rng;
use std::{thread::current, vec};

#[inline]
pub fn decompress(compressed_data: &mut Vec<u32>, block_size: usize) -> Vec<u8> {
    let mut hasher: Hasher;

    //strip and process bitmask
    let byte_list = process_char_bitmask(&compressed_data.drain(0..8).as_slice().to_vec());
    println!("{:?}", byte_list);
    println!("{}", byte_list.len());

    let mut decompressed_data: Vec<u8> = vec![0; block_size * compressed_data.len()];

    let mut matches: usize = 0; //only here to chech if there are collisions
    let mut current_hash: u32 = 0;
    let mut current_bytes: Vec<u8> = vec![0; block_size];
    loop {
        hasher = Hasher::new();
        hasher.update(current_bytes.as_slice());
        current_hash = hasher.finalize();
        for (i, item) in compressed_data.iter().enumerate() {
            if current_hash == *item {
                for ii in 0..block_size {
                    decompressed_data[(i * block_size) + ii] = current_bytes[ii];
                }
                matches += 1;
            }
        }
        if !increment_byte_vector(&mut current_bytes) {
            break;
        }
    }

    println!(
        "Avg matches per hash: {}",
        matches as f64 / compressed_data.len() as f64
    );

    decompressed_data
}

fn increment_byte_vector(vector: &mut Vec<u8>) -> bool {
    let mut carry = true;
    let mut i = 0;
    {
        let mut will_overflow: bool = true;
        for i in vector.iter() {
            if *i != 255 {
                will_overflow = false;
            }
        }
        if will_overflow == true {
            return false;
        }
    }

    while carry {
        if (vector[i] == 255) && carry {
            vector[i] = 0;
        } else {
            vector[i] += 1;
            carry = false;
        }
        i += 1;
    }
    true
}

fn generate_random_byte_vector(size: usize) -> Vec<u8> {
    let mut to_ret = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        to_ret.push(rng.gen::<u8>());
    }
    to_ret
}

fn process_char_bitmask(bitmask: &Vec<u32>) -> Vec<u8> {
    if bitmask.len() != 8 {
        panic!("bitmask length not correct");
    }
    let mut to_ret: Vec<u8> = Vec::with_capacity(256);
    for (i, item) in bitmask.iter().enumerate() {
        for bit in 0..32 {
            if (*item & (1 << bit)) != 0 {
                to_ret.push(((i*32)+bit) as u8);
            }
        }
    }

    to_ret.shrink_to_fit();
    to_ret.sort();
    to_ret
}
