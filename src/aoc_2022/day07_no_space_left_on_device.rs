use self::Node::*;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::rc::{Rc, Weak};

fn part_1(input: &str) -> u64 {
    let root = build_fs(input);
    let mut nodes = vec![root];
    let mut sum = 0;
    while !nodes.is_empty() {
        nodes = nodes
            .iter()
            .filter(|n| matches!(***n, Dir { .. }))
            .inspect(|n| {
                let size = n.dir_info().unwrap().0.get().unwrap();
                if size < 100000 {
                    sum += size;
                }
            })
            .map(|n| {
                let (.., entrys) = n.dir_info().unwrap();
                entrys
                    .borrow()
                    .values()
                    .map(|rc| rc.clone())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let root = build_fs(input);
    let used = root.dir_info().unwrap().0.get().unwrap();
    let need = 30000000 - (70000000 - used);

    let mut nodes = vec![root];
    let mut min_delete = used;
    while !nodes.is_empty() {
        nodes = nodes
            .iter()
            .filter(|n| matches!(***n, Dir { .. }))
            .inspect(|n| {
                let size = n.dir_info().unwrap().0.get().unwrap();
                if size > need && size < min_delete {
                    min_delete = size;
                }
            })
            .map(|n| {
                let (.., entrys) = n.dir_info().unwrap();
                entrys
                    .borrow()
                    .values()
                    .map(|rc| rc.clone())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
    }
    min_delete
}

enum Node {
    Dir {
        size: Cell<Option<u64>>,
        parent: Option<Weak<Node>>,
        entrys: RefCell<HashMap<String, Rc<Node>>>,
    },
    File(u64),
}

impl Node {
    fn dir_info<'a>(
        self: &'a Rc<Self>,
    ) -> Option<(
        &'a Cell<Option<u64>>,
        &'a Option<Weak<Node>>,
        &'a RefCell<HashMap<String, Rc<Node>>>,
    )> {
        if let Dir {
            size,
            parent,
            entrys,
        } = &**self
        {
            return Some((size, parent, entrys));
        }
        None
    }

    fn cal_dir_size(self: &Rc<Self>) {
        let (size, _, entrys) = self.dir_info().unwrap();
        if size.get().is_some() {
            return;
        }
        let mut sum = 0;
        for node in entrys.borrow().values() {
            match &**node {
                File(n) => sum += *n,
                Dir { size: s, .. } if s.get().is_some() => sum += s.get().unwrap(),
                _ => return,
            }
        }
        size.set(Some(sum));
    }
}

fn build_fs(input: &str) -> Rc<Node> {
    let root = Rc::new(Dir {
        parent: None,
        entrys: RefCell::new(HashMap::new()),
        size: Cell::new(None),
    });
    let mut wd = root.clone();
    for line in input.trim().lines().skip(1) {
        match line {
            "$ ls" => continue,
            "$ cd /" => {
                wd.cal_dir_size();
                wd = root.clone();
                continue;
            }
            "$ cd .." => {
                wd.cal_dir_size();
                let parent = wd.dir_info().unwrap().1.as_ref();
                wd = parent.map(|p| p.upgrade().unwrap()).unwrap_or(root.clone());
                continue;
            }
            _ => (),
        }
        let (left, right) = line.rsplit_once(' ').unwrap();
        match left {
            "$ cd" => {
                let (.., entrys) = wd.dir_info().unwrap();
                let n = entrys.borrow().get(right).unwrap().clone();
                wd = n;
            }
            "dir" => {
                let (.., entrys) = wd.dir_info().unwrap();
                entrys.borrow_mut().entry(right.into()).or_insert_with(|| {
                    Rc::new(Dir {
                        parent: Some(Rc::downgrade(&wd)),
                        entrys: RefCell::new(HashMap::new()),
                        size: Cell::new(None),
                    })
                });
            }
            _ => {
                let mut entrys = wd.dir_info().unwrap().2.borrow_mut();
                entrys
                    .entry(right.into())
                    .or_insert_with(|| Rc::new(File(left.parse().unwrap())));
            }
        }
    }
    while !Rc::ptr_eq(&wd, &root) {
        wd.cal_dir_size();
        let parent = wd.dir_info().unwrap().1.as_ref();
        wd = parent.map(|p| p.upgrade().unwrap()).unwrap_or(root.clone())
    }
    root.cal_dir_size();
    root
}

pub fn print_answer() {
    let input = read_to_string("./src/aoc_2022/data/input_2022_07").unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&input.trim()),
        part_2(&input.trim())
    )
}

#[test]
fn part_1_test() {
    let input: &str = "$ cd /\n\
                       $ ls\n\
                       dir a\n\
                       14848514 b.txt\n\
                       8504156 c.dat\n\
                       dir d\n\
                       $ cd a\n\
                       $ ls\n\
                       dir e\n\
                       29116 f\n\
                       2557 g\n\
                       62596 h.lst\n\
                       $ cd e\n\
                       $ ls\n\
                       584 i\n\
                       $ cd ..\n\
                       $ cd ..\n\
                       $ cd d\n\
                       $ ls\n\
                       4060174 j\n\
                       8033020 d.log\n\
                       5626152 d.ext\n\
                       7214296 k";
    assert_eq!(part_1(input), 95437);
}

#[test]
fn part_2_test() {
    let input: &str = "$ cd /\n\
                       $ ls\n\
                       dir a\n\
                       14848514 b.txt\n\
                       8504156 c.dat\n\
                       dir d\n\
                       $ cd a\n\
                       $ ls\n\
                       dir e\n\
                       29116 f\n\
                       2557 g\n\
                       62596 h.lst\n\
                       $ cd e\n\
                       $ ls\n\
                       584 i\n\
                       $ cd ..\n\
                       $ cd ..\n\
                       $ cd d\n\
                       $ ls\n\
                       4060174 j\n\
                       8033020 d.log\n\
                       5626152 d.ext\n\
                       7214296 k";
    assert_eq!(part_2(input), 24933642);
}
