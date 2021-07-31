use std::path::{Path, PathBuf};

pub struct Hook {
    pub path: PathBuf,
}

impl Hook {
    pub fn find_hook(path: &Path, hook_type: HookType) -> Option<Self> {
        if !path.is_dir() {
            return None;
        }

        let hooks_dir = path.join(Path::new("git-hooks"));

        if !hooks_dir.is_dir() {
            return None;
        }

        let hook_file = hooks_dir.join(hook_type.filename());

        if !hook_file.exists() {
            return None;
        }

        Some(Hook { path: hook_file })
    }
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
