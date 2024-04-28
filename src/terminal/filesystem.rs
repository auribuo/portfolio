use std::sync::atomic::AtomicU64;

use anyhow::{Context, Result};
use lazy_static::lazy_static;

use crate::projects;

lazy_static! {
    static ref NEXT_NODE_ID: AtomicU64 = AtomicU64::new(0);
}

pub(crate) fn next_node_id() -> u64 {
    NEXT_NODE_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[derive(Debug, Clone)]
pub(crate) enum FsEntryType {
    Directory(Vec<FsEntry>),
    File(String),
    Link(u64),
}

#[derive(Debug, Clone)]
pub(crate) struct FsEntry {
    id: u64,
    name: &'static str,
    full_path: &'static str,
    accessible: bool,
    ty: FsEntryType,
}

impl FsEntry {
    pub(crate) fn new(
        name: &'static str,
        full_path: &'static str,
        accessible: bool,
        ty: FsEntryType,
    ) -> Self {
        Self {
            id: next_node_id(),
            name,
            full_path,
            accessible,
            ty,
        }
    }

    pub(crate) fn new_dir(full_path: &'static str, children: Vec<FsEntry>) -> Self {
        let name = full_path
            .split("/")
            .last()
            .expect("Malformed path. Fix it!");
        Self {
            id: next_node_id(),
            name,
            full_path,
            accessible: true,
            ty: FsEntryType::Directory(children),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Filesystem {
    root: FsEntry,
    cwd: u64,
}

impl Filesystem {
    pub(crate) fn new() -> Self {
        let mut fs = Self {
            root: FsEntry::new(
                "/",
                "/",
                true,
                FsEntryType::Directory(vec![FsEntry::new_dir(
                    "/home",
                    vec![FsEntry::new_dir(
                        "/home/user",
                        vec![projects::gen_project_directory()],
                    )],
                )]),
            ),
            cwd: 0,
        };
        let home_id = Self::find(&fs.root, |node| node.full_path == "/home/user");
        fs.cwd = home_id.expect("Hardcoded directories should exist").id;
        fs
    }

    fn find<'a, F>(root: &'a FsEntry, func: F) -> Option<&'a FsEntry>
    where
        F: Fn(&'a FsEntry) -> bool + Clone,
    {
        if func(&root) {
            return Some(&root);
        }

        if let FsEntryType::Directory(children) = &root.ty {
            for node in children {
                if let Some(node) = Self::find(node, func.clone()) {
                    return Some(node);
                }
            }
        }
        None
    }

    pub(crate) fn cwd(&self) -> &'static str {
        match Self::find(&self.root, |node| node.id == self.cwd) {
            Some(node) => node.full_path,
            None => "",
        }
    }

    fn get_cwd(&self) -> &FsEntry {
        Self::find(&self.root, |node| node.id == self.cwd)
            .context("Searching for cwd")
            .unwrap()
    }

    fn get_parent(&self, node: &FsEntry) -> Option<&FsEntry> {
        Self::find(&self.root, |n: &FsEntry| {
            if let FsEntryType::Directory(children) = &n.ty {
                for child in children {
                    if child.id == node.id {
                        return true;
                    }
                }
            }
            return false;
        })
    }

    pub(crate) fn cd(&mut self, dir: String) -> Result<(), String> {
        let splits = dir.split("/");
        for dir_name in splits {
            let mut new_current_dir: u64 = 0;
            let cwd = self.get_cwd();
            match dir_name {
                "." => {
                    continue;
                }
                ".." => {
                    if let Some(new_dir) = self.get_parent(cwd) {
                        new_current_dir = new_dir.id;
                    } else {
                        return Err(format!("no such directory: {}", dir));
                    }
                }
                dir_name => match &cwd.ty {
                    FsEntryType::Directory(children) => {
                        let mut has_matched = false;
                        for entry in children {
                            match entry.ty {
                                FsEntryType::Directory(_) => {
                                    if entry.name == dir_name {
                                        if !entry.accessible {
                                            return Err(format!("forbidden access: {}", dir));
                                        }
                                        new_current_dir = entry.id;
                                        has_matched = true;
                                    }
                                }
                                FsEntryType::File(_) => {
                                    return Err(format!("not a directory: {}", dir))
                                }
                                FsEntryType::Link(_) => {
                                    if entry.name == dir_name {
                                        if !entry.accessible {
                                            return Err(format!("forbidden access: {}", dir));
                                        }
                                        match self.follow_link(&entry) {
                                                Some(link_dest) => match link_dest.ty {
                                                    FsEntryType::Directory(_) => {
                                                        if !link_dest.accessible {
                                                            return Err(format!(
                                                                "forbidden access: {}",
                                                                dir
                                                            ));
                                                        }
                                                        new_current_dir = link_dest.id;
                                                        has_matched = true;
                                                    }
                                                    FsEntryType::File(_) => {
                                                        return Err(format!("not a directory: {}", dir))
                                                    }
                                                    _ => unreachable!("After follow_link the node can only be a file or directory"),
                                                },
                                                None => return Err("broken link".to_string()),
                                            }
                                    }
                                }
                            }
                        }
                        if !has_matched {
                            return Err(format!("no such directory: {}", dir));
                        }
                    }
                    _ => unreachable!(),
                },
            }
            self.cwd = new_current_dir;
        }

        Ok(())
    }

    pub(crate) fn ls(&self) -> Result<Vec<LsResult>, String> {
        let mut results = Vec::<LsResult>::new();
        let cwd = self.get_cwd();
        match &cwd.ty {
            FsEntryType::Directory(contents) => {
                for entry in contents {
                    results.push(LsResult::from(entry))
                }
            }
            _ => unreachable!(),
        }
        Ok(results)
    }

    fn follow_link<'a>(&'a self, link: &'a FsEntry) -> Option<&'a FsEntry> {
        if let FsEntryType::Link(to) = link.ty {
            let to = Self::find(&self.root, |node| node.id == to)?;
            if let FsEntryType::Link(_) = to.ty {
                self.follow_link(to)
            } else {
                Some(to)
            }
        } else {
            Some(link)
        }
    }
}

fn map_accessible(is_accessible: bool) -> String {
    if is_accessible {
        "rwx".to_string()
    } else {
        "---".to_string()
    }
}

fn type_flag(ty: &FsEntryType) -> &'static str {
    match ty {
        FsEntryType::File(_) => ".",
        FsEntryType::Directory(_) => "d",
        FsEntryType::Link(_) => "l",
    }
}

pub(crate) enum LsResultType {
    File,
    Directory,
    Link,
}

impl From<&FsEntryType> for LsResultType {
    fn from(value: &FsEntryType) -> Self {
        match value {
            FsEntryType::Directory(_) => Self::Directory,
            FsEntryType::File(_) => Self::File,
            FsEntryType::Link(_) => Self::Link,
        }
    }
}

impl From<&FsEntry> for LsResult {
    fn from(value: &FsEntry) -> Self {
        let mut res = Self {
            name: value.name,
            permissions: format!(
                "{}{}",
                type_flag(&value.ty),
                map_accessible(value.accessible)
            ),
            size: None,
            ty: LsResultType::from(&value.ty),
        };
        if let FsEntryType::File(contents) = &value.ty {
            res.size = Some(contents.len() as u64)
        }
        res
    }
}

pub(crate) struct LsResult {
    permissions: String,
    name: &'static str,
    size: Option<u64>,
    ty: LsResultType,
}

impl LsResult {
    pub(crate) fn size(&self) -> Option<u64> {
        self.size
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn ty(&self) -> &LsResultType {
        &self.ty
    }

    pub(crate) fn permissions(&self) -> &str {
        &self.permissions
    }
}
