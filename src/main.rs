use std::{fs, env, process::exit,path::Path};

const STUDENT_NUMBER: u32 = numbergoeshere;
const FIRST_NAME: &str = "first name goes here";
const LAST_NAME: &str = "last name goes here";
const PATH: &str = "folder goes here";

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("Usage: submit <File>");
        exit(128);
    }

    let destination: String = PATH.to_string() + &STUDENT_NUMBER.to_string() +"_"+ LAST_NAME +"_"+ FIRST_NAME +"_"+&args[1].to_string();

    if Path::new(PATH).exists() == false{
        print!("Directory missing, Creating now.");
        fs::create_dir_all(PATH).expect("Tried to create directory and failed, check permissions.");
    }

    if Path::new(&destination).exists() == true{
        println!("File already present in directory, Aborting.");
        exit(17);
    }else {
        fs::copy(&args[1], destination).expect("Failed to copy file.");
    }

    

}

