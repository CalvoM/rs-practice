use std::{cell::RefCell, fs, rc::Rc};

const CMD_CD_STARTER: &str = "$ cd";
const CMD_LS_STARTER: &str = "$ ls";
const BACK_DIR: &str = "..";
const ROOT_DIR: &str = "/";

#[derive(Debug, Clone)]
struct File {
    pub name: String,
    pub size: usize,
}
#[derive(Default, Debug, Clone)]
struct Folder {
    pub name: String,
    pub files: Vec<File>,
    pub folders: Vec<Folder>,
    pub size: usize,
}

pub type FolderRef = Rc<RefCell<Folder>>;
impl Folder {
    fn new(name: String) -> Self {
        Folder {
            name,
            files: Vec::new(),
            folders: Vec::new(),
            size: 0,
        }
    }
}
pub fn question_1() {
    let content = fs::read_to_string("./src/day_seven/testinput.txt").unwrap();
    let content = content.trim_end();
    let lines = content.split('\n').collect::<Vec<&str>>();
    println!("{:#?}", lines);
    tree_traversal(lines);
}

fn tree_traversal(lines: Vec<&str>) {
    let mut opened_folders: Vec<FolderRef> = Vec::new();
    for line in lines {
        if !opened_folders.is_empty() {
            let current_dir = opened_folders.last_mut().unwrap();
        }
        if line.starts_with(CMD_CD_STARTER) {
            let name = line.split(' ').collect::<Vec<&str>>()[2];
            let folder = Rc::new(RefCell::new(Folder::new(name.to_owned())));
            opened_folders.push(folder);
        } else if line.starts_with(CMD_LS_STARTER) {
            line;
        }
    }
}
