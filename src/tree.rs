use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::Command;

#[derive(Debug)]
pub struct BST<T>
where
    T: Ord + Debug + Copy,
{
    value: Option<T>,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>,
}

impl<T> BST<T>
where
    T: Ord + Debug + Copy,
{
    pub fn new() -> BST<T> {
        BST {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn from(value: T) -> BST<T> {
        BST {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        match &self.value {
            Some(key) => {
                let target = match value.cmp(&key) {
                    Ordering::Less => &mut self.left,
                    Ordering::Equal => &mut self.left,
                    Ordering::Greater => &mut self.right,
                };
                match target {
                    Some(ref mut node) => {
                        node.insert(value);
                    }
                    None => {
                        let node: BST<T> = BST::from(value);
                        *target = Some(Box::new(node));
                    }
                }
            }
            None => self.value = Some(value),
        }
    }

    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => match &self.value {
                Some(value) => Some(&value),
                None => None,
            },
        }
    }

    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => match &self.value {
                Some(value) => Some(&value),
                None => None,
            },
        }
    }

    pub fn search(&self, target: &T) -> bool {
        match &self.value {
            Some(key) => match &target.cmp(key) {
                Ordering::Equal => return true,
                Ordering::Less => match &self.left {
                    Some(node) => node.search(target),
                    None => false,
                },
                Ordering::Greater => match &self.right {
                    Some(node) => node.search(target),
                    None => false,
                },
            },
            None => return false,
        }
    }

    pub fn to_graphviz(&self, filename: &str) {
        let path = format!("graphviz/{}.dot", filename);
        let output = format!("graphviz/{}.png", filename);
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .expect("Cannot open file.");

        f.set_len(0).expect("Failed to truncate.");
        f.write(b"graph {\n").expect("Failed to write.");
        self.traverse_graphviz(&mut f);
        f.write(b"}").expect("Failed to write.");

        Command::new("dot")
            .args(&["-Tpng", &path, "-o", &output])
            .spawn()
            .expect("Failed to execute PNG conversion.");
    }

    fn traverse_graphviz(&self, f: &mut File) {
        match &self.left {
            Some(node) => {
                f.write_all(
                    format!(
                        "\t{:?} -- {:?}\n",
                        &self.value.unwrap(),
                        &node.value.unwrap()
                    )
                    .as_bytes(),
                )
                .expect("Couldn't write to file.");
                node.traverse_graphviz(f);
            }
            None => (),
        }
        match &self.right {
            Some(node) => {
                f.write_all(
                    format!(
                        "\t{:?} -- {:?}\n",
                        &self.value.unwrap(),
                        &node.value.unwrap()
                    )
                    .as_bytes(),
                )
                .expect("Couldn't write to file.");
                node.traverse_graphviz(f);
            }
            None => (),
        }
    }
}
