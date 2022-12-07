use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
pub enum Log {
    Command(Command),
    File(usize, String),
    Dir(String),
}

fn dirs(v: &[Log]) -> HashMap<PathBuf, usize> {
    let mut path = PathBuf::new();
    let mut files: Vec<_> = v
        .iter()
        .flat_map(|log| match log {
            Log::Command(Command::Cd(s)) if s == "/" => {
                path = PathBuf::from("/");
                None
            }
            Log::Command(Command::Cd(s)) if s == ".." => {
                path.pop();
                None
            }
            Log::Command(Command::Cd(s)) => {
                path = path.join(s);
                None
            }
            Log::Command(Command::Ls) => None,
            Log::File(size, f) => Some((path.join(f), size)),
            Log::Dir(_) => None,
        })
        .collect();

    files.sort();
    files.dedup();

    let mut dirs: HashMap<_, usize> = HashMap::new();

    files.iter().for_each(|(p, size)| {
        for anc in p.ancestors().skip(1) {
            *dirs.entry(anc.to_path_buf()).or_insert(0) += *size;
        }
    });

    dirs
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 7;
    type Input1 = Vec<Log>;
    type Input2 = Vec<Log>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|l| {
                if let Some(command) = l.strip_prefix("$ ") {
                    if let Some(arg) = command.strip_prefix("cd ") {
                        Log::Command(Command::Cd(arg.to_string()))
                    } else {
                        Log::Command(Command::Ls)
                    }
                } else if let Some(dir) = l.strip_prefix("dir ") {
                    Log::Dir(dir.to_string())
                } else {
                    let (size, file) = l.split_once(' ').unwrap();
                    Log::File(size.parse().unwrap(), file.to_string())
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let dirs = dirs(v);
        println!("{:#?}", dirs);
        dirs.iter()
            .filter(|(_p, x)| **x <= 100000)
            .inspect(|(p, x)| println!("{:?} {:?}", p, x))
            .map(|(_, x)| *x)
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let total_space = 70000000;
        let needed = 30000000;

        let dirs = dirs(v);

        let used = dirs.get(Path::new("/")).unwrap();
        let delete_size = needed - (total_space - used);

        let delete = dirs
            .iter()
            .filter(|(_p, x)| **x >= delete_size)
            .min_by_key(|(_, v)| **v)
            .unwrap();

        *delete.1
    }
}

crate::default_tests!(1513699, 7991939);
crate::path_tests!(
    [(sol1, "test/day07.txt", 95437)],
    [(sol2, "test/day07.txt", 24933642)]
);
