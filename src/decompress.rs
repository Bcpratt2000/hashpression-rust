use hash32::FnvHasher;
use hash32::Hasher;
use rand::Rng;

pub fn decompress(compressed_data: &Vec<u32>, block_size: usize) -> Vec<u8> {
    let mut hasher: FnvHasher; // = FnvHasher::default();

    let mut decompressed_data: Vec<u8> = Vec::with_capacity(block_size * compressed_data.len());

    let mut current_hash: u32 = 0;
    let mut current_bytes: Vec<u8>;

    for i in compressed_data.iter() {
        current_bytes = vec![0; block_size];
        hasher = FnvHasher::default();
        hasher.write(current_bytes.as_slice());
        current_hash = hasher.finish();
        while *i != current_hash {
            increment_byte_vector(&mut current_bytes);
            hasher = FnvHasher::default();
            hasher.write(current_bytes.as_slice());
            current_hash = hasher.finish();
            // println!("Hash: {}", current_hash);
            // println!("Bytes: {:?}", current_bytes);
        }
        println!("{:?}", current_bytes);
        decompressed_data.append(&mut current_bytes);
    }
    decompressed_data
}

fn increment_byte_vector(vector: &mut Vec<u8>) {
    let mut carry = true;
    let mut i = 0;
    while carry {
        if (vector[i] == 255) && carry {
            vector[i] = 0;
        } else {
            vector[i] += 1;
            carry = false;
        }
        i += 1;
    }
}

fn generate_random_byte_vector(size: usize) -> Vec<u8> {
    let mut to_ret = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        to_ret.push(rng.gen::<u8>());
    }
    to_ret
}
