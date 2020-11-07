use rand::Rng;
mod quick_sort;
use quick_sort::quick_sort;

fn main() {
	const LENGTH: i32 = 10;
	let mut data = make_vec(LENGTH);

	data = quick_sort(data);

	println!("{:?}", data);
}

fn make_vec(length: i32) -> Vec<i32> {
	let mut rng = rand::thread_rng();
	let mut vector = Vec::new();
	for _ in 0..length {
		let x: f32 = rng.gen();
		vector.push((x / 0.01) as i32);
	}
	vector
}
