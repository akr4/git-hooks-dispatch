use crate::args::Opt;
use crate::hook::{Hook, HookType};
use anyhow::Result;
use git2::Status;
use std::path::Path;
use structopt::StructOpt;

mod args;
mod hook;

fn main() -> Result<()> {
    env_logger::init();

    let envvars: Vec<(String, String)> = std::env::vars().collect();
    let envvars: Vec<(String, String)> = envvars
        .into_iter()
        .filter(|(k, _)| k.starts_with("GIT_"))
        .collect();
    log::debug!("{:#?}", envvars);

    let opt = Opt::from_args();
    log::debug!("{:#?}", opt);

    let hook_type = HookType::from_name(&opt.hook);
    if hook_type.is_none() {
        log::error!("invalid hook name {}", opt.hook);
        std::process::exit(1);
    }
    let hook_type = hook_type.unwrap();

    let repo = git2::Repository::open(Path::new(".")).unwrap();
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
            let path = Path::new(x.as_os_str());
            if !path.exists() {
                continue;
            }
            let path_str = path
                .to_str()
                .ok_or(anyhow::Error::msg("failed to convert path to string"))?;
            log::debug!("searching hook for {}", path_str);
            let hook = Hook::find_hook(path, hook_type);
            if let Some(hook) = hook {
                let hook_path_str = hook
                    .path
                    .to_str()
                    .ok_or(anyhow::Error::msg("failed to convert path to string"))?;
                log::debug!("found hook {}", hook_path_str);
                log::info!("executing hook ({})", hook_path_str);
                let status = hook.exec(&opt.args)?;
                if !status.success() {
                    log::error!(
                        "hook exit with status code ({})",
                        status.code().map(|x| x.to_string()).unwrap_or("unknown".to_string())
                    );
                    std::process::exit(status.code().unwrap_or(1));
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
    #[test]
    fn should_exit_if_hook_exit_with_error() {}
}
