use std::fs;
use std::path;

#[allow(dead_code)]
pub fn create_files() {
    let my_path = "./data/file01.txt";
    let my_path2 = "./data/file02.txt";
    let my_path3 = "./data/file03.txt";
    let text = "I am a Rust Developer 🦀.";
    let text2 = "I am a Solana Developer 🦀.";
    let text3 = "I am a Python Developer ✅.";
    _ = fs::write(my_path, text);

    _ = fs::write(my_path2, text2);
    _ = fs::write(my_path3, text3);

    _ = fs::remove_file(my_path3);
}

#[allow(dead_code)]
pub fn remove_dir() {
    let path = "./data";
    _ = fs::remove_dir_all(path);
}

#[allow(dead_code)]
pub fn test_create_dir() {
    let path = "./data";
    let my_path = path::Path::new(path);
    if my_path.exists() {
        println!("Directory already exist skipping creation...");
        return;
    }
    let create_dir_result = fs::create_dir(path);

    if create_dir_result.is_ok() {
        println!("Created new dir successfully");
    } else {
        println!(
            "Some problem occured creating directory. {:?}",
            create_dir_result.err().unwrap()
        );
    }
}

#[allow(dead_code)]
pub fn read_from_file() {
    let file_to_read = "./data/file02.txt";
    let result = fs::read_to_string(file_to_read);

    // If you want to read u8 bytes and convert it to string using the char and iterator method , though read_to_string is better
    // let result = fs::read(file_to_read);
    // let convert_byte_to_string = |mut a: String, v: &u8| {
    //     let new_char = char::from(*v);
    //     _ = a.push(new_char);
    //     return a;
    // };

    // if result.is_ok() {
    //     println!(
    //         "{:?}",
    //         result
    //             .ok()
    //             .unwrap()
    //             .iter()
    //             .fold(String::from(""), convert_byte_to_string)
    //     );
    // } else {
    //     println!("Data does not exist!")
    // }

    if result.is_ok() {
        println!("{:?}", result.unwrap());
    } else {
        println!("Data does not exist!")
    }
}
