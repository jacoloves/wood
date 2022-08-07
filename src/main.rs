use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";

    if args.len() >= 2 {
        target_dir = &args[1];
    }

    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, 0, 0);
}

fn tree(target: &path::PathBuf, level: isize, last_depth: isize) {
   let files = target.read_dir().expect("not exist path");
   let file_cnts = files.count();
   let for_files = target.read_dir().expect("not exist path");
   let mut file_nums = 0;
   for ent in for_files {
       file_nums += 1;
       let path = ent.unwrap().path();
        
       for _ in 1..=level-last_depth {
            print!("│   ");
       }

       for _ in 1..=last_depth {
            print!("    ");
       }

       let fname = path.file_name().unwrap().to_string_lossy();

       if file_nums == file_cnts {
            println!("└── {}", fname);
       } else {
            println!("├── {}", fname);
       }

       if path.is_dir() {
            if file_nums == file_cnts {
                tree(&path, level+1, last_depth+1);
            } else {
                tree(&path, level+1, last_depth);
            }
            continue;
       }
                       
   }

}
