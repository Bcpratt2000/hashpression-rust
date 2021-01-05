use crc32fast::Hasher;
use std::collections::HashMap;
use std::mem;
use std::time::Instant;

use crate::file_io;

#[inline]
pub fn decompress(compressed_data: &mut Vec<u32>, block_size: usize) -> Vec<u8> {
	let mut hasher: Hasher;

	//strip and process bitmask
	let byte_list = process_char_bitmask(&compressed_data.drain(0..8).as_slice().to_vec());

	//strip and save file checksum
	let checksum = compressed_data.drain(0..1).as_slice()[0];
	// println!("{}", checksum);

	//allocate space for decompressed information
	let mut decompressed_data: Vec<u8> = vec![0; block_size * compressed_data.len()];
	let start = Instant::now();
	let hashmap = hashmap_from_vector(&compressed_data);
	println!(
		"HashMap generated in {}",
		start.elapsed().as_millis() as f64 / 1000 as f64
	);

	drop(compressed_data);

	let mut matches: usize = 0; //here to check for collisions
	let mut current_hash: u32 = 0;
	let mut current_bytes: Vec<u8> = vec![0; block_size];
	let mut current_byte_tracker: Vec<u8> = vec![0; block_size];
	loop {
		hasher = Hasher::new();

		// generate current byte list using the indecies tracked in current_byte_tracker
		for i in 0..block_size {
			current_bytes[i] = byte_list[current_byte_tracker[i] as usize];
		}

		hasher.update(current_bytes.as_slice());
		current_hash = hasher.finalize();

		if hashmap.contains_key(&current_hash) {
			for i in hashmap.get(&current_hash).unwrap().iter() {
				for ii in 0..block_size {
					decompressed_data[(i * block_size) + ii] = current_bytes[ii];
				}
				matches += 1;
			}
		}

		if matches >= decompressed_data.len() / block_size {
			hasher = Hasher::new();
			hasher.update(decompressed_data.as_slice());
			if checksum == hasher.finalize() {
				println!("Checksum Matched");
				break;
			}
		}
		if !increment_byte_vector_max(&mut current_byte_tracker, byte_list.len() as u8) {
			hasher = Hasher::new();
			hasher.update(decompressed_data.as_slice());
			println!("Unable to decompress accurately");
			break;
		}
	}

	println!(
		"Avg matches per hash: {}",
		matches as f64 / (decompressed_data.len() / block_size) as f64
	);
	println!("{} {}", matches, (decompressed_data.len() / block_size));

	decompressed_data
}

fn hashmap_from_vector(vector: &Vec<u32>) -> HashMap<u32, Vec<usize>> {
	let mut hashmap: HashMap<u32, Vec<usize>> = HashMap::with_capacity(vector.len());

	for (i, item) in vector.iter().enumerate() {
		if hashmap.contains_key(item) {
			hashmap.get_mut(item).unwrap().push(i);
		} else {
			hashmap.insert(*item, vec![i]);
		}
	}
	hashmap
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

fn increment_byte_vector_max(vector: &mut Vec<u8>, max: u8) -> bool {
	let mut carry = true;
	let mut i = 0;
	{
		let mut will_overflow: bool = true;
		for i in vector.iter() {
			if *i != max - 1 {
				will_overflow = false;
			}
		}
		if will_overflow == true {
			return false;
		}
	}

	while carry {
		if (vector[i] == max - 1) && carry {
			vector[i] = 0;
		} else {
			vector[i] += 1;
			carry = false;
		}
		i += 1;
	}
	// println!("{:?}", vector);
	true
}

fn process_char_bitmask(bitmask: &Vec<u32>) -> Vec<u8> {
	if bitmask.len() != 8 {
		panic!("bitmask length not correct");
	}
	let mut to_ret: Vec<u8> = Vec::with_capacity(256);
	for (i, item) in bitmask.iter().enumerate() {
		for bit in 0..32 {
			if (*item & (1 << bit)) != 0 {
				to_ret.push(((i * 32) + bit) as u8);
			}
		}
	}
	to_ret.shrink_to_fit();
	to_ret.sort();
	to_ret
}
