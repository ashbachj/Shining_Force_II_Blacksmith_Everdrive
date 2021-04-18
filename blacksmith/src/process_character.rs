use std::io;

pub mod item_manager;

fn get_user_input(buffer: &mut Vec<u8>, item_offset: usize) {
   loop {
       let mut input = String::new();
       println!("Edit current items? (0-3), n to continue, q to quit");
       io::stdin().read_line(&mut input).ok()
            .expect("Couldn't read line");
       if input.trim() == "n" || input.trim() == "" {
       	  return
       } else if input.trim() == "q" {
       	  std::process::exit(0);
       }
       let item_number = input.trim().parse::<usize>().unwrap();
       if item_number < 4 {
          let mut new_item_input = String::new();
	  let current_item = item_manager::get_item(buffer[item_offset+item_number*2] as usize, false);
       	  println!("Smith {} to?", current_item);
	  io::stdin().read_line(&mut new_item_input).ok()
	  			     .expect("Couldn't read line");
	  println!("{}", new_item_input);
	  let new_item = new_item_input.trim().parse::<usize>().unwrap();
	  println!("{} is now {}", current_item, item_manager::get_item(new_item as usize, false));
	  buffer[item_offset+2*item_number] = new_item as u8;	 
       }
   }
}

pub fn process_character(buffer: &mut Vec<u8>, number: i32) {
    if number == 0 {
       item_manager::get_item(0, true);
    }
    let character_data_size = 56;
    let character_offset = (0xe800 + character_data_size*number) as usize;
    let item_offset = character_offset + 33;

    let name_buffer = &buffer[character_offset..character_offset+7];
    let name = std::str::from_utf8(name_buffer).unwrap().to_string();
    println!("{:?}", name.trim_matches(char::from(0)));
    for current_item in 0..4 {
    	let current_item_number: usize = buffer[item_offset+current_item*2].into();
	let item = item_manager::get_item(current_item_number, false);
	println!("\tItem {}: {}", current_item, item);
    }
    get_user_input(buffer, item_offset);
}
