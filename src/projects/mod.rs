use crate::terminal::filesystem::FsEntry;

pub(crate) fn gen_project_directory() -> FsEntry {
    FsEntry::new_dir("/home/user/projects", vec![])
}
