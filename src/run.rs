use crate::executor::Executor;
use crate::hook::Hook;
use anyhow::Result;
use git2::Status;
use std::path::Path;

type StatusCode = i32;

pub fn run<P: AsRef<Path>, E: Executor>(
    repo_dir: P,
    hook_name: &str,
    args: &Vec<String>,
    executor: &E,
    hooks_dir_names: Vec<String>,
) -> Result<StatusCode> {
    debug_assert!(repo_dir.as_ref().is_absolute());

    log::debug!("hooks_dir_names = {:?}", hooks_dir_names);

    let repo = git2::Repository::open(repo_dir.as_ref().clone()).unwrap();
    for e in repo.statuses(None).unwrap().iter() {
        let git_entry_path_str = e
            .path()
            .ok_or(anyhow::Error::msg("failed to convert path to string"))?;
        let status = e.status();
        log::debug!("found git entry: {} ({:?})", git_entry_path_str, status);
        if !is_changed(status) {
            continue;
        }
        log::debug!("found changed git entry: {}", git_entry_path_str);
        let git_entry_path = Path::new(git_entry_path_str);

        for path in git_entry_path.ancestors() {
            log::debug!("testing path: {}", path.display());
            let abs_path = repo_dir.as_ref().join(path);
            if !abs_path.exists() {
                continue;
            }
            if abs_path.canonicalize()? == repo_dir.as_ref().canonicalize()? {
                log::debug!("found the repo dir");
                break;
            }
            log::debug!(
                "searching hook in {}",
                abs_path
                    .to_str()
                    .ok_or(anyhow::Error::msg("failed to convert path to string"))?
            );
            let hook = Hook::find_hook(&abs_path, hook_name, &hooks_dir_names);
            if let Some(hook) = hook {
                let hook_path_str = hook
                    .path
                    .to_str()
                    .ok_or(anyhow::Error::msg("failed to convert path to string"))?;
                log::debug!("found hook {}", hook_path_str);
                log::info!(
                    "executing hook ({}) in ({})",
                    hook_path_str,
                    abs_path.display()
                );
                let status = executor.execute(&abs_path, &hook.path, args)?;
                if status != 0 {
                    log::error!("hook exit with status code ({})", status);
                    return Ok(status);
                }
            }
        }
    }

    Ok(0)
}

fn is_changed(status: Status) -> bool {
    status.is_index_deleted()
        || status.is_index_modified()
        || status.is_index_new()
        || status.is_index_renamed()
        || status.is_index_typechange()
        || status.is_wt_deleted()
        || status.is_wt_modified()
        || status.is_wt_renamed()
        || status.is_wt_typechange()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::executor::MockExecutor;
    use anyhow::Result;
    use mockall::Sequence;
    use std::io::Write;
    use std::path::Path;
    use tempdir::TempDir;

    #[test]
    fn should_run_hook() -> Result<()> {
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

        let working_dir_abs_path = repo_dir.path().join("1");

        let mut mock = MockExecutor::new();
        mock.expect_execute()
            .times(1)
            .withf(
                move |working_dir: &Path, hook_path: &Path, args: &Vec<String>| {
                    working_dir == working_dir_abs_path
                        && hook_path == pre_commit_abs_path
                        && args.len() == 0
                },
            )
            .returning(|_, _, _| Ok(0));

        run(
            repo_dir.path(),
            "pre-commit",
            &vec![],
            &mock,
            vec!["git-hooks".to_string()],
        )?;

        Ok(())
    }

    #[test]
    fn should_run_hook_recursively() -> Result<()> {
        let repo_dir = TempDir::new("repo")?;

        std::fs::create_dir(repo_dir.path().join("1"))?;
        std::fs::create_dir(repo_dir.path().join("1/git-hooks"))?;
        std::fs::create_dir(repo_dir.path().join("1/2"))?;
        std::fs::create_dir(repo_dir.path().join("1/2/git-hooks"))?;

        let a = Path::new("1/2/a");
        std::fs::File::create(repo_dir.path().join(a))?.write_all("a".as_bytes())?;

        let pre_commit_abs_path1 = repo_dir.path().join("1/git-hooks/pre-commit");
        std::fs::File::create(pre_commit_abs_path1.as_path())?;
        let pre_commit_abs_path2 = repo_dir.path().join("1/2/git-hooks/pre-commit");
        std::fs::File::create(pre_commit_abs_path2.as_path())?;

        let repo = git2::Repository::init(repo_dir.path()).unwrap();
        let mut index = repo.index()?;
        index.add_all(vec![a], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let working_dir_abs_path1 = repo_dir.path().join("1");
        let working_dir_abs_path2 = repo_dir.path().join("1/2");

        let mut seq = Sequence::new();
        let mut mock = MockExecutor::new();
        mock.expect_execute()
            .withf(
                move |working_dir: &Path, hook_path: &Path, args: &Vec<String>| {
                    working_dir == working_dir_abs_path2
                        && hook_path == pre_commit_abs_path2
                        && args.len() == 0
                },
            )
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _, _| Ok(0));
        mock.expect_execute()
            .withf(
                move |working_dir: &Path, hook_path: &Path, args: &Vec<String>| {
                    working_dir == working_dir_abs_path1
                        && hook_path == pre_commit_abs_path1
                        && args.len() == 0
                },
            )
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _, _| Ok(0));

        run(
            repo_dir.path(),
            "pre-commit",
            &vec![],
            &mock,
            vec!["git-hooks".to_string()],
        )?;

        Ok(())
    }

    #[test]
    fn should_run_hook_once() -> Result<()> {
        let repo_dir = TempDir::new("repo")?;

        std::fs::create_dir(repo_dir.path().join("1"))?;
        std::fs::create_dir(repo_dir.path().join("1/git-hooks"))?;

        let a = Path::new("1/a");
        std::fs::File::create(repo_dir.path().join(a))?.write_all("a".as_bytes())?;
        let b = Path::new("1/b");
        std::fs::File::create(repo_dir.path().join(a))?.write_all("a".as_bytes())?;

        let pre_commit_abs_path = repo_dir.path().join("1/git-hooks/pre-commit");
        std::fs::File::create(pre_commit_abs_path.as_path())?;

        let repo = git2::Repository::init(repo_dir.path()).unwrap();
        let mut index = repo.index()?;
        index.add_all(vec![a, b], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let mut mock = MockExecutor::new();
        mock.expect_execute().times(1).returning(|_, _, _| Ok(0));

        run(
            repo_dir.path(),
            "pre-commit",
            &vec![],
            &mock,
            vec!["git-hooks".to_string()],
        )?;

        Ok(())
    }

    #[test]
    fn should_exit_if_hook_exit_with_error() -> Result<()> {
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
        mock.expect_execute().times(1).returning(|_, _, _| Ok(1));

        let status_code = run(
            repo_dir.path(),
            "pre-commit",
            &vec![],
            &mock,
            vec!["git-hooks".to_string()],
        )?;

        assert_eq!(status_code, 1);

        Ok(())
    }

    #[test]
    fn should_not_execute_hook_in_root_directory() -> Result<()> {
        let repo_dir = TempDir::new("repo")?;

        std::fs::create_dir(repo_dir.path().join("git-hooks"))?;
        std::fs::File::create(repo_dir.path().join("git-hooks/pre-commit"))?;

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

        let working_dir_abs_path = repo_dir.path().join("1");

        let mut mock = MockExecutor::new();
        mock.expect_execute()
            .withf(
                move |working_dir: &Path, hook_path: &Path, args: &Vec<String>| {
                    working_dir == working_dir_abs_path
                        && hook_path == pre_commit_abs_path
                        && args.len() == 0
                },
            )
            .times(1)
            .returning(|_, _, _| Ok(0));

        run(
            repo_dir.path(),
            "pre-commit",
            &vec![],
            &mock,
            vec!["git-hooks".to_string()],
        )?;

        Ok(())
    }
}
