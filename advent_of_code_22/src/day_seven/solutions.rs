use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Debug, Clone)]
struct Folder<'a> {
    pub name: String,
    pub files: Vec<File>,
    pub folders: Vec<Folder<'a>>,
    pub parent: Option<&'a Box<Folder<'a>>>,
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
fn question_1() {
    let content = fs::read_to_string("./src/day_seven/testinput.txt").unwrap();
    let content = content.trim_end();
    let lines: Vec<&str> = content.split('\n').collect();
    let index = 0;
    tree_traversal(lines, index);
}

fn tree_traversal(lines: Vec<&str>, mut index: usize) {
    let mut root_folder = Box::new(Folder {
        name: String::new(),
        files: vec![],
        folders: vec![],
        parent: None,
    });
    index = 0;
    let mut cmds = lines[index].split(' ').collect::<Vec<&str>>();
    if cmds[0] == CMD_STARTER && cmds[1] == "cd" {
        root_folder.name = cmds[2].to_owned();
        index += 1;
    }
    cmds = lines[index].split(' ').collect::<Vec<&str>>();
    if cmds[0] == CMD_STARTER && cmds[1] == "ls" {
        index += 1;
    }
    cmds = lines[index].split(' ').collect::<Vec<&str>>();
    while cmds[0] != CMD_STARTER {
        //Check if dir
        if cmds[0] == "dir" {
            let folder_found = Folder {
                name: cmds[1].to_owned(),
                folders: vec![],
                files: vec![],
                parent: Some(&root_folder),
            };
            root_folder.folders.push(folder_found);
        } else {
            let size: usize = cmds[0].parse().unwrap();
            let file = File {
                name: cmds[1].to_owned(),
                size,
            };
            root_folder.files.push(file)
        }
        index += 1;
        cmds = lines[index].split(' ').collect::<Vec<&str>>();
    }
    create_tree(lines, &mut index, &mut root_folder);

    println!("{:?}", root_folder.folders);
}

fn create_tree<'a>(
    lines: Vec<&'a str>,
    index: &'a mut usize,
    parent_folder: &'a mut Folder<'a>,
) -> Option<Box<Folder<'a>>> {
    println!("\tParent {:?}\n", parent_folder);
    let mut folder = Box::new(Folder {
        name: String::new(),
        files: vec![],
        folders: vec![],
        parent: None,
    });
    let mut cmds = lines[*index].split(' ').collect::<Vec<&str>>();
    // if moving to parent directory
    if cmds[0] == CMD_STARTER && cmds[cmds.len() - 1] == BACK_DIR {
        return None;
    }
    if cmds[0] == CMD_STARTER && cmds[1] == "cd" {
        folder.name = cmds[2].to_owned();
        println!("-->{}\n", folder.name);
        *index += 1;
    }
    cmds = lines[*index].split(' ').collect::<Vec<&str>>();
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
                parent: Some(&folder),
            };
            println!("\tDir {:?}", folder_found);
            folder.folders.push(folder_found);
        } else {
            let size: usize = cmds[0].parse().unwrap();
            let file = File {
                name: cmds[1].to_owned(),
                size,
            };
            println!("\tFile {:?}", file);
            folder.files.push(file)
        }
        *index += 1;
        cmds = lines[*index].split(' ').collect::<Vec<&str>>();
    }
    let mut folder_index = 1_001;
    for (i, f) in parent_folder.folders.iter_mut().enumerate() {
        if f.name == folder.name {
            folder_index = i;
        }
    }
    if folder_index != 1_001 {
        parent_folder.folders[folder_index] = *folder.clone();
    }

    let ret = create_tree(lines.clone(), index, &mut folder).is_none();
    if ret {
        println!("Going back from child: {:?}", folder);
        create_tree(lines.clone(), index, parent_folder);
        let mut cmds = lines[*index].split(' ').collect::<Vec<&str>>();
        // if moving to parent directory
        if cmds[0] == CMD_STARTER && cmds[cmds.len() - 1] == BACK_DIR {
            return None;
        } else {
            create_tree(lines.clone(), index, parent_folder);
        }
    }
    Some(folder)
}
