use comfy_table::Table;
use serde::Serialize;
use serde_json::Value;

pub fn loading(event: &str, index: usize, max: usize) {
	let index = index + 1;
	print!("\r{event} ... ({}/{})", index, max);
	if index == max {
		println!("\nDone!");
	}
	// idea: add zero padding
}

pub fn table<T: Serialize>(records: &Vec<T>, headers: Vec<&str>) -> String {
	let mut rows: Vec<Vec<String>> = vec![];

	for record in records {
		let mut cells: Vec<String> = vec![];

		let record = serde_json::to_value(record).expect("cannot convert record to json value");
		let record = record
			.as_object()
			.expect("cannot convert record value to json object")
			.to_owned();

		let empty_cell = Value::String(String::new());
		for header in &headers {
			let cell = record.get(*header).unwrap_or(&empty_cell).to_string();

			let cell = if cell.is_empty() || cell == *"null" {
				String::from("---")
			} else if cell.starts_with('"') && cell.ends_with('"') {
				cell[1..cell.len() - 1].to_owned()
			} else {
				cell
			};

			cells.push(cell);
		}

		rows.push(cells);
	}

	let headers = headers.iter().map(|x| x.to_uppercase());

	Table::new().set_header(headers).add_rows(rows).to_string()
}
