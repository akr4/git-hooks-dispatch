use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Hook {
    pub path: PathBuf,
    /// corresponding project dir path (`/foo/bar` for `/foo/bar/.git-repo/pre-commit`)
    pub base_dir: PathBuf,
}

impl Hook {
    pub fn find_hook(path: &Path, hook_type: &str, hooks_dir_names: &Vec<String>) -> Option<Self> {
        if !path.is_dir() {
            return None;
        }

        let hooks_dir = find_hooks_dir(path, hooks_dir_names);
        if hooks_dir.is_none() {
            return None;
        }
        let hooks_dir = hooks_dir.unwrap();

        let hook_file = hooks_dir.join(hook_type);

        if !hook_file.exists() {
            return None;
        }

        debug_assert!(hook_file.is_absolute());

        Some(Hook {
            path: hook_file,
            base_dir: path.to_path_buf(),
        })
    }
}

fn find_hooks_dir(path: &Path, names: &Vec<String>) -> Option<PathBuf> {
    for d in names {
        let hooks_dir = path.join(Path::new(d.as_str()));

        if hooks_dir.is_dir() {
            return Some(hooks_dir);
        }
    }

    None
}
