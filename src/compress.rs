use std::collections::HashMap;

use crc32fast::Hasher;


pub fn compress(block_size: usize, input_vector: &mut Vec<u8>) -> Vec<u32> {
    //pad end with spaces to be divisible by block_size
    if input_vector.len() % block_size != 0 {
        for _ in 0..block_size - (input_vector.len() % block_size) {
            input_vector.push(0);
        }
    }

    //declare vairables
    let mut to_ret: Vec<u32> = Vec::new();
    let mut hasher;

    for i in 0..(input_vector.len() / block_size) {
        //initalize hasher every loop because hasher.finish() does not clear it
        hasher = Hasher::new();

        //write block to hash to hasher
        hasher.update(
            input_vector
                .get(block_size * i..block_size * i + block_size)
                .unwrap(),
        );
        //write hash as a new entry in the vector
        to_ret.push(hasher.finalize());
    }
    to_ret
}


pub fn generate_char_bitmask(input: &Vec<u8>) -> Vec<u8>{
    let mut tracker: [usize; 256] = [0; 256];

    for i in input.iter(){
        tracker[*i as usize]+=1;
    }

    let mut ret_vec: Vec<u8> = vec!(0; 32);

    for i in 0..255{
        if tracker[i] != 0{
            ret_vec[i/32] += (1 <<i%32);
        }
    }
    ret_vec
}
