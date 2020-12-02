use hash32::FnvHasher;
use hash32::Hasher;

pub fn compress(block_size: usize, s: &mut String) -> Vec<u32>{
	//pad end with spaces to be divisible by block_size
	for _ in 0..block_size-(s.len()%block_size){
		s.push(' ');
	}

	//declare vairables
	let mut to_ret: Vec<u32> = Vec::new();
	let mut hasher;

	for i in 1..s.len()/block_size{
		//initalize hasher every loop because hasher.finish() does not clear it
		hasher = FnvHasher::default();

		//write block to hash to hasher
		hasher.write(s.get(block_size*i..block_size*i+block_size)
							.unwrap()
							.as_bytes());
		//write hash as a new entry in the vector
		to_ret.push(hasher.finish());
	}
	to_ret
}
