use std::path::{Path, PathBuf};

pub struct Hook {
    pub path: PathBuf,
}

impl Hook {
    pub fn find_hook(path: &Path, hook_type: HookType, hooks_dir_names: &Vec<String>) -> Option<Self> {
        if !path.is_dir() {
            return None;
        }

        let hooks_dir = find_hooks_dir(path, hooks_dir_names);
        if hooks_dir.is_none() {
            return None;
        }
        let hooks_dir = hooks_dir.unwrap();

        let hook_file = hooks_dir.join(hook_type.filename());

        if !hook_file.exists() {
            return None;
        }

        Some(Hook { path: hook_file })
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

#[derive(Copy, Clone)]
pub enum HookType {
    PreCommit,
    PostRewrite,
}

impl HookType {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "pre-commit" => Some(HookType::PreCommit),
            "post-rewrite" => Some(HookType::PostRewrite),
            _ => None,
        }
    }

    pub fn filename(&self) -> String {
        match self {
            Self::PreCommit => "pre-commit".to_string(),
            Self::PostRewrite => "post-rewrite".to_string(),
        }
    }
}
