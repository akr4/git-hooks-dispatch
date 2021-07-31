use crate::executor::Executor;
use crate::hook::{Hook, HookType};
use anyhow::Result;
use git2::Status;
use std::path::Path;

pub fn run<P: AsRef<Path>, E: Executor>(
    repo_dir: P,
    hook_name: &str,
    args: &Vec<String>,
    executor: &E,
) -> Result<()> {
    let hook_type = HookType::from_name(hook_name);
    if hook_type.is_none() {
        log::error!("invalid hook name {}", hook_name);
        std::process::exit(1);
    }
    let hook_type = hook_type.unwrap();

    let repo = git2::Repository::open(repo_dir.as_ref().clone()).unwrap();
    for e in repo.statuses(None).unwrap().iter() {
        let path_str = e
            .path()
            .ok_or(anyhow::Error::msg("failed to convert path to string"))?;
        let status = e.status();
        log::debug!("found git entry: {} ({:?})", path_str, status);
        if !is_changed(status) {
            continue;
        }
        log::debug!("found changed git entry: {}", path_str);
        let path = Path::new(path_str);

        for x in path.ancestors() {
            let path_from_root = repo_dir.as_ref().join(x);
            if !path_from_root.exists() {
                continue;
            }
            log::debug!(
                "searching hook in {}",
                path_from_root
                    .to_str()
                    .ok_or(anyhow::Error::msg("failed to convert path to string"))?
            );
            let hook = Hook::find_hook(&path_from_root, hook_type);
            if let Some(hook) = hook {
                let hook_path_str = hook
                    .path
                    .to_str()
                    .ok_or(anyhow::Error::msg("failed to convert path to string"))?;
                log::debug!("found hook {}", hook_path_str);
                log::info!("executing hook ({})", hook_path_str);
                let status = executor.execute(&hook.path, args)?;
                if status != 0 {
                    log::error!("hook exit with status code ({})", status);
                    std::process::exit(status);
                }
            }
        }
    }

    Ok(())
}

fn is_changed(status: Status) -> bool {
    status.is_index_deleted()
        || status.is_index_modified()
        || status.is_index_new()
        || status.is_index_renamed()
        || status.is_index_typechange()
        || status.is_wt_deleted()
        || status.is_wt_modified()
        // || status.is_wt_new()
        || status.is_wt_renamed()
        || status.is_wt_typechange()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::executor::MockExecutor;
    use anyhow::Result;
    use std::io::Write;
    use std::path::Path;
    use tempdir::TempDir;

    #[test]
    fn should_run_hook() -> Result<()> {
        env_logger::init();

        let repo_dir = TempDir::new("repo")?;

        std::fs::create_dir(repo_dir.path().join("1"))?;
        std::fs::create_dir(repo_dir.path().join("1/git-hooks"))?;

        let a = Path::new("1/a");
        std::fs::File::create(repo_dir.path().join(a))?.write_all("a".as_bytes())?;

        let pre_commit_abs_path = repo_dir.path().join("1/git-hooks/pre-commit");
        std::fs::File::create(pre_commit_abs_path.as_path())?;

        let repo = git2::Repository::init(repo_dir.path()).unwrap();
        let mut index = repo.index()?;
        index.add_all(vec![a], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let mut mock = MockExecutor::new();
        mock.expect_execute()
            .times(1)
            .withf(move |path: &Path, args: &Vec<String>| {
                path == pre_commit_abs_path && args.len() == 0
            })
            .returning(|_, _| Ok(0));

        run(repo_dir.path(), "pre-commit", &vec![], &mock)?;

        Ok(())
    }

    #[test]
    fn should_run_hook_recursively() {
        todo!()
    }

    #[test]
    fn should_run_hook_once() {
        todo!()
    }

    #[test]
    fn should_exit_if_hook_exit_with_error() {
        todo!()
    }
}
