use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";

    if args.len() >= 2 {
        target_dir = &args[1];
    }

    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, "");
}

fn tree(target: &path::PathBuf, prefix_str: &str) {
   let files = target.read_dir().expect("not exist path");
   let file_cnts = files.count();
   let for_files = target.read_dir().expect("not exist path");
   let mut file_nums = 0;
   for ent in for_files {
       file_nums += 1;
       let path = ent.unwrap().path();
        
       let fname = path.file_name().unwrap().to_string_lossy();

       if file_nums == file_cnts {
            println!("{}└── {}", prefix_str, fname);
       } else {
            println!("{}├── {}", prefix_str, fname);
       }

       let tmp_str = prefix_str.to_string();

       if path.is_dir() {
            if file_nums == file_cnts {
                let add_str = "    ";
                let arg_str = tmp_str + add_str;
                tree(&path, &arg_str);
            } else {
                let add_str = "│   ";
                let arg_str = tmp_str + add_str;
                tree(&path, &arg_str);
            }
            continue;
       }
                       
   }
}
