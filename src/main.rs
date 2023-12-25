use std::env;
use std::path::Path;
use std::process::exit;
use comfy_table::Table;

static IGNORE_DIRS: [&str; 3] = ["dev", "proc", "sys"]; 

fn main() {
    let mut starting_path = "/";//starting path for searching files and folders
    let args: Vec<String> = env::args().collect();
    let mut files_matched: Vec<String> = Vec::new();
    let search_term = args.get(1);
    let mut fname: String = "".to_string();
    let mut exclude_dirs: bool = false;
    match search_term {
        Some(path) => {fname = path.to_owned()},
        None => {println!("No name specified!\nAbort!"); exit(0)}, 
    }

    if fname.contains("--help") {
        println!("This is locate but written in rust.\n Usage: loc [filename] [args].\n Args: 1) --home-only - searches only in $HOME (false by default).\n 2) --exclude-dirs - searches only for files and not directories (false by default)");
        exit(0);
    }

    if args.contains(&("--exclude-dirs".to_string())) {
        exclude_dirs = true;
    }
    if args.contains(&("--home-only".to_string())) {
        starting_path = "/home";
    }

    get_fname(&fname, starting_path, &mut files_matched, exclude_dirs);
    pprint(files_matched);
}

fn get_fname(fname: &String, dirname: &str, files_matched: &mut Vec<String>, exclude_dirs: bool) {
    //println!("{}", &dirname);
    let paths = Path::new(dirname);
    let all_paths = paths.read_dir();
    // match all_paths {
    //     Ok(all_paths) => all_paths,
    //     Err(e) => Err(e),
    // };
    if let Ok(paths) = all_paths {
        for path in paths {
            //if let Ok(path) = path {
            if let Ok(path) = path {
                //println!("{:#?}", path.path());
                if IGNORE_DIRS.contains(&path.path().to_string_lossy().split('/').collect::<Vec<&str>>()[1]) {}
                else {
                if path.path().is_dir() {
                    if path.path().to_string_lossy().rsplit_once('/').unwrap().1.contains(fname) {
                        if exclude_dirs {} else {
                        files_matched.push(path.path().to_string_lossy().to_string())
                        }
                    }
                    get_fname(&fname, path.path().to_str().expect("ERROR WHILE READING DIRS"), files_matched, exclude_dirs)
                } else if path.path().is_file() {
                    if path.path().to_string_lossy().rsplit_once('/').unwrap().1.contains(fname) {
                        files_matched.push(path.path().to_string_lossy().to_string())
                    }
                }
            }
            }
        }
    }
}

fn pprint(files_matched: Vec<String>) {
    println!("Found {} paths", files_matched.len());
    let mut table = Table::new();
    table.set_header(vec!["TYPE", "PATH"]);
    for i in files_matched {
        if Path::new(&i).is_dir() {
            table.add_row(vec!["DIR", &i]);
        } else if Path::new(&i).is_file() {
            table.add_row(vec!["FILE", &i]);            
        }
    }
    println!("{table}");
}
