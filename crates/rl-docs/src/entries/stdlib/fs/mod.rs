use crate::entry::{FnEntry, StdEntry};

mod copy_file;
mod file_accessed;
mod file_created;
mod file_modified;
mod file_permissions;
mod file_size;
mod glob;
mod hardlink;
mod list_dir;
mod list_dir_names;
mod lock_file;
mod mkdir;
mod mkdir_all;
mod move_file;
mod realpath;
mod readlink;
mod rename_file;
mod rmdir;
mod rmdir_all;
mod set_permissions;
mod symlink;
mod temp_dir;
mod temp_file;
mod temp_file_in;
mod touch;
mod truncate_file;
mod unlock_file;
mod walk_dir;

pub static FS: StdEntry = StdEntry {
    name: "fs",
    description: "functions for working with the filesystem",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &copy_file::COPY_FILE,
    &file_accessed::FILE_ACCESSED,
    &file_created::FILE_CREATED,
    &file_modified::FILE_MODIFIED,
    &file_permissions::FILE_PERMISSIONS,
    &file_size::FILE_SIZE,
    &glob::GLOB,
    &hardlink::HARDLINK,
    &list_dir::LIST_DIR,
    &list_dir_names::LIST_DIR_NAMES,
    &lock_file::LOCK_FILE,
    &mkdir::MKDIR,
    &mkdir_all::MKDIR_ALL,
    &move_file::MOVE_FILE,
    &realpath::REALPATH,
    &readlink::READLINK,
    &rename_file::RENAME_FILE,
    &rmdir::RMDIR,
    &rmdir_all::RMDIR_ALL,
    &set_permissions::SET_PERMISSIONS,
    &symlink::SYMLINK,
    &temp_dir::TEMP_DIR,
    &temp_file::TEMP_FILE,
    &temp_file_in::TEMP_FILE_IN,
    &touch::TOUCH,
    &truncate_file::TRUNCATE_FILE,
    &unlock_file::UNLOCK_FILE,
    &walk_dir::WALK_DIR,
];
