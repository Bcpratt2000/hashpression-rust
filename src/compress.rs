use crc32fast::Hasher;

/*
Outputed file data structure
bytes 0..31   = bitmask of used bytes in the original file
bytes 32..36  = crc32 hash of the original file
bytes 37..end = crc32 hashed data in 4 byte segments
*/
pub fn compress(block_size: usize, input_vector: &mut Vec<u8>) -> Vec<u32> {
    //pad end with spaces to be divisible by block_size
    if input_vector.len() % block_size != 0 {
        for _ in 0..block_size - (input_vector.len() % block_size) {
            input_vector.push(0);
        }
    }

    //declare vairables
    let mut to_ret: Vec<u32> = Vec::with_capacity(100 + (input_vector.len() / block_size));
    let mut hasher;

    {
        let mut checksum: Hasher = Hasher::new();
        checksum.update(input_vector);

        //prepend byte bitmask
        let char_bitmask = generate_char_bitmask(&input_vector);
        for i in char_bitmask {
            to_ret.push(i);
        }

        //prepend file checksum after byte mask, this is calculated after padding is added
        to_ret.push(checksum.finalize());
    }

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

pub fn generate_char_bitmask(input: &Vec<u8>) -> Vec<u32> {
    let mut tracker: [usize; 256] = [0; 256];

    for i in input.iter() {
        tracker[*i as usize] += 1;
    }

    let mut ret_vec: Vec<u32> = vec![0; 8];

    for i in 0..256 {
        if tracker[i] != 0 {
            ret_vec[i / 32] += 1 << i % 32;
        }
    }
    ret_vec
}
