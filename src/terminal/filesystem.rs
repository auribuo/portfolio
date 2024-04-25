use dioxus::html::tr;

#[derive(Debug, Clone)]
pub(crate) enum FsEntry {
    Directory {
        id: u64,
        name: &'static str,
        full_path: &'static str,
        accessible: bool,
        contents: Vec<FsEntry>,
    },
    File {
        id: u64,
        name: &'static str,
        full_path: &'static str,
        accessible: bool,
        contents: String,
    },
    Link {
        id: u64,
        name: &'static str,
        full_path: &'static str,
        accessible: bool,
        to: u64,
    },
}

impl FsEntry {
    pub(crate) fn full_path(&self) -> String {
        match self {
            FsEntry::Directory { full_path, .. } => full_path.to_string(),
            FsEntry::File { full_path, .. } => full_path.to_string(),
            FsEntry::Link { full_path, .. } => full_path.to_string(),
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
        Self {
            root: FsEntry::Directory {
                id: 0,
                name: "/",
                full_path: "/",
                accessible: true,
                contents: vec![FsEntry::Directory {
                    id: 1,
                    name: "home",
                    full_path: "/home",
                    accessible: true,
                    contents: vec![
                        FsEntry::Directory {
                            id: 2,
                            name: "user",
                            full_path: "/home/user",
                            accessible: true,
                            contents: vec![FsEntry::Link {
                                id: 4,
                                name: "..",
                                full_path: "/home/user/..",
                                accessible: true,
                                to: 1,
                            }],
                        },
                        FsEntry::Link {
                            id: 3,
                            name: "..",
                            full_path: "/home/..",
                            accessible: true,
                            to: 0,
                        },
                    ],
                }],
            },
            cwd: 2,
        }
    }

    pub(crate) fn cwd(&self) -> String {
        match self.find_cwd() {
            FsEntry::Directory { full_path, .. } => full_path.to_string(),
            _ => "".to_string(),
        }
    }

    fn find_cwd(&self) -> &FsEntry {
        match Self::find_id(&self.root, self.cwd) {
            Some(dir) => dir,
            _ => unreachable!(),
        }
    }

    fn find_id(root: &FsEntry, find: u64) -> Option<&FsEntry> {
        return if let FsEntry::Directory { id, contents, .. } = root {
            return if *id == find {
                Some(root)
            } else {
                for entry in contents {
                    match entry {
                        FsEntry::Directory { .. } => {
                            if let Some(res) = Self::find_id(entry, find) {
                                return Some(res);
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                return None;
            };
        } else {
            None
        };
    }

    pub(crate) fn cd(&mut self, dir: String) -> Result<(), String> {
        let splits = dir.split("/");

        for dir_name in splits {
            let mut new_current_dir: u64 = 0;
            let cwd = self.find_cwd();
            info!("{:?} {:?}", cwd, dir);
            match cwd {
                FsEntry::Directory { contents, .. } => {
                    let mut has_matched = false;
                    for entry in contents {
                        match entry {
                            FsEntry::Directory {
                                id,
                                name,
                                accessible,
                                ..
                            } => {
                                if *name == dir_name {
                                    if !accessible {
                                        return Err(format!("forbidden access: {}", dir));
                                    }
                                    new_current_dir = *id;
                                    has_matched = true;
                                }
                            }
                            FsEntry::File { .. } => {
                                return Err(format!("not a directory: {}", dir))
                            }
                            FsEntry::Link {
                                name, accessible, ..
                            } => {
                                if *name == dir_name {
                                    if !accessible {
                                        return Err(format!("forbidden access: {}", dir));
                                    }
                                    match self.follow_link(entry) {
                                        Some(link_dest) => match link_dest {
                                            FsEntry::Directory { id, accessible, .. } => {
                                                if !accessible {
                                                    return Err(format!(
                                                        "forbidden access: {}",
                                                        dir
                                                    ));
                                                }
                                                new_current_dir = *id;
                                                has_matched = true;
                                            }
                                            FsEntry::File { .. } => {
                                                return Err(format!("not a directory: {}", dir))
                                            }
                                            _ => unimplemented!(),
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
            }
            self.cwd = new_current_dir;
        }

        Ok(())
    }

    pub(crate) fn ls(&self) -> Result<Vec<LsResult>, String> {
        let mut results = Vec::<LsResult>::new();
        let cwd = self.find_cwd();
        match cwd {
            FsEntry::Directory { contents, .. } => {
                for entry in contents {
                    match entry {
                        FsEntry::Directory {
                            name, accessible, ..
                        } => results.push(LsResult {
                            permissions: format!("d{}", map_accessible(*accessible)),
                            name: format!("{name}/"),
                            size: None,
                            ty: LsResultType::Directory,
                        }),
                        FsEntry::File {
                            name,
                            accessible,
                            contents,
                            ..
                        } => results.push(LsResult {
                            permissions: format!(".{}", map_accessible(*accessible)),
                            name: name.to_string(),
                            size: Some(contents.len() as u64),
                            ty: LsResultType::File,
                        }),
                        FsEntry::Link {
                            accessible, name, ..
                        } => results.push(LsResult {
                            permissions: format!("l{}", map_accessible(*accessible)),
                            name: format!(
                                "{} -> {}",
                                name,
                                self.follow_link(cwd)
                                    .map_or("?".to_string(), |fe| fe.full_path().to_string())
                            ),
                            size: None,
                            ty: LsResultType::Link,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
        Ok(results)
    }

    fn follow_link<'a>(&'a self, link: &'a FsEntry) -> Option<&'a FsEntry> {
        if let FsEntry::Link { to, .. } = link {
            let to = Self::find_id(&self.root, *to)?;
            if let FsEntry::Link { .. } = to {
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

pub(crate) enum LsResultType {
    File,
    Directory,
    Link,
}

pub(crate) struct LsResult {
    pub(crate) permissions: String,
    pub(crate) name: String,
    pub(crate) size: Option<u64>,
    pub(crate) ty: LsResultType,
}
