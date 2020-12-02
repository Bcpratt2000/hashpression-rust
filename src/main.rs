pub mod compress;
fn main() {
	let mut s = String::from("Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World! Hell");
    let compressed_vector: Vec<u32> = compress::compress(64, &mut s);
    for i in compressed_vector.iter(){
		println!("{}", i);
	}
}
