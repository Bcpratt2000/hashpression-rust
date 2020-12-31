use hash32::FnvHasher;
use hash32::Hasher;

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
        hasher = FnvHasher::default();

        //write block to hash to hasher
        hasher.write(
            input_vector
                .get(block_size * i..block_size * i + block_size)
                .unwrap(),
        );
        //write hash as a new entry in the vector
        to_ret.push(hasher.finish());
    }
    to_ret
}
