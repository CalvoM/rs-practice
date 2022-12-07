use std::fs;

#[derive(Debug)]
struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Debug)]
struct Folder {
    pub name: String,
    pub files: Vec<File>,
    pub folders: Vec<Folder>,
}

const CMD_STARTER: &str = "$";
const BACK_DIR: &str = "..";
const ROOT_DIR: &str = "/";

// Logic
// When we encounter cd command, we start a folder with name
// We read the next line
// If it is cd again, create the folder as child of earlier folder
// If it is ls, then get the next lines and save as contents of parent folder
// We read the following lines with the files in the folder
pub fn question_1() {
    let content = fs::read_to_string("./src/day_seven/testinput.txt").unwrap();
    let content = content.trim_end();
    let lines: Vec<&str> = content.split('\n').collect();
    let index = 0;
    tree_traversal(lines, index);
}

fn tree_traversal(lines: Vec<&str>, mut index: usize) -> Folder {
    let mut root_folder = Folder {
        name: String::new(),
        files: vec![],
        folders: vec![],
    };
    index = 0;
    create_tree(lines, &mut index);
    println!("{}", index);
    root_folder
}

fn create_tree(lines: Vec<&str>, index: &mut usize) -> Option<Folder> {
    let mut folder = Folder {
        name: String::new(),
        files: vec![],
        folders: vec![],
    };
    let mut cmds = lines[*index].split(' ').collect::<Vec<&str>>();
    // if moving to parent directory
    if cmds[0] == CMD_STARTER && cmds[cmds.len() - 1] == BACK_DIR {
        return None;
    }
    if cmds[0] == CMD_STARTER && cmds[1] == "cd" {
        folder.name = cmds[2].to_owned();
        *index += 1;
    }
    cmds = lines[*index].split(' ').collect::<Vec<&str>>();
    //TODO: We might need logic for ls cmd
    // Reading files and folders
    if cmds[0] == CMD_STARTER && cmds[1] == "ls" {
        *index += 1;
    }
    cmds = lines[*index].split(' ').collect::<Vec<&str>>();
    while cmds[0] != CMD_STARTER {
        //Check if dir
        if cmds[0] == "dir" {
            let folder_found = Folder {
                name: cmds[1].to_owned(),
                folders: vec![],
                files: vec![],
            };
            folder.folders.push(folder_found);
        } else {
            let size: usize = cmds[0].parse().unwrap();
            let file = File {
                name: cmds[1].to_owned(),
                size: size,
            };
            folder.files.push(file)
        }
        *index += 1;
        cmds = lines[*index].split(' ').collect::<Vec<&str>>();
    }
    println!("{:?}", folder);
    Some(folder)
}
