pub mod add_fn
{
	// PACKAGES
	use std::fs;
	use std::io::Write;


	// COPY FILE'S CONTENT and CREATE SAVE
	fn create_save(unq_name: &String, file_path: &String)
	{
		fn copy_file(file_path: &String) -> String { return fs::read_to_string(file_path).expect("Should have been able to read the file"); }

		let file_name = format!("rustix/saves/{:02}.txt", unq_name);
		let data = copy_file(&file_path);
		let new_data = data.as_bytes();

    let mut f = fs::File::create(file_name).expect("Unable to create file");
    f.write_all(new_data).expect("Unable to write data");
	}


	// START POINT
	pub fn start(file_path: &String, unq_name: &String)
	{
		crate::log::logger::start("ADD   ".to_string());

		let TIME_DATE : [String; 2] = crate::time::time_fn::start();

		let save_info : [String; 4] = [
			file_path.to_string(),
			unq_name.to_string(),
			TIME_DATE[0].to_string(),
			TIME_DATE[1].to_string()];


		let CREATE_COPY = crate::database::add::start(save_info);
		if (CREATE_COPY == true) { create_save(&unq_name, &file_path); }
		else { println!(":("); }
	}
}