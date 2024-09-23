pub fn loading(event: &str, index: usize, max: usize) {
	let index = index + 1;
	print!("\r{event} ... ({}/{})", index, max);
	if index == max {
		println!("\nDone!");
	}
	// idea: add zero padding
}
