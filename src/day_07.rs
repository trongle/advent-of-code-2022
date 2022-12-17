use std::{cell::RefCell, rc::Rc};

pub fn day_07() {
    let mut system = System::new();
    let content = include_str!("day_07.txt")
        .split("\n")
        .filter(|line| !line.is_empty());

    content.for_each(|line| {
        system.do_action(line);
    });

    let result_1 = find(system.dir_stack.first().unwrap().as_ref().borrow().clone())
        .iter()
        .fold(0, |acc, dir| acc + dir.get_size());

    println!("{}", result_1);
}

fn find(dir: Directory) -> Vec<Directory> {
    let mut vec1 = dir
        .directories
        .iter()
        .filter(|d| d.as_ref().borrow().get_size() <= 100000)
        .map(|d| d.as_ref().borrow().clone())
        .collect::<Vec<Directory>>();

    let mut vec2 = dir
        .clone()
        .directories
        .iter()
        .flat_map(|d| find(d.as_ref().borrow().clone()))
        .collect::<Vec<Directory>>();

    vec1.append(&mut vec2);

    vec1
}

struct System {
    dir_stack: Vec<Rc<RefCell<Directory>>>,
}

#[derive(Debug, PartialEq, Clone)]
struct Directory {
    name: String,
    directories: Vec<Rc<RefCell<Directory>>>,
    size: usize,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            directories: Vec::new(),
            size: 0,
        }
    }

    fn traverse(&mut self, dir_name: &str) -> Rc<RefCell<Directory>> {
        let directory = self
            .directories
            .iter()
            .find(|dir| dir.as_ref().borrow_mut().name == dir_name);

        if directory.is_none() {
            let directory = Rc::new(RefCell::new(Directory::new(dir_name)));
            self.directories.push(Rc::clone(&directory));
            Rc::clone(&directory)
        } else {
            Rc::clone(directory.unwrap())
        }
    }

    fn add_size(&mut self, size: usize) {
        self.size += size;
    }

    fn get_size(&self) -> usize {
        self.size
            + self
                .directories
                .clone()
                .into_iter()
                .fold(0, |acc, dir| acc + dir.as_ref().borrow_mut().get_size())
    }
}

impl System {
    pub fn new() -> Self {
        System {
            dir_stack: vec![Rc::new(RefCell::new(Directory::new("/")))],
        }
    }

    pub fn do_action(&mut self, input: &str) {
        if input.starts_with("$ ls") || input.starts_with("dir") {
        } else if input.starts_with("$ cd /") {
            self.dir_stack = vec![Rc::clone(&self.dir_stack.first().unwrap())];
        } else if input.starts_with("$ cd ..") {
            self.dir_stack.pop();
        } else if input.starts_with("$ cd ") {
            let dir_name = input.replace("$ cd ", "");
            let dir = self
                .current_dir()
                .unwrap()
                .as_ref()
                .borrow_mut()
                .traverse(&dir_name);
            self.dir_stack.push(dir);
        } else {
            let size: usize = input.split_whitespace().nth(0).unwrap().parse().unwrap();
            self.current_dir()
                .unwrap()
                .as_ref()
                .borrow_mut()
                .add_size(size);
        }
    }

    fn current_dir(&self) -> Option<&Rc<RefCell<Directory>>> {
        self.dir_stack.last()
    }
}
