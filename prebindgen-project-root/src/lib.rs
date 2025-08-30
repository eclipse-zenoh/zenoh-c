use std::path::PathBuf;

/// Returns the absolute path to the cargo workspace root that built this crate.
///
/// This works only if this crate is included in a Cargo workspace. In the opposite case,
/// it will return an error, explaining how to correctly configure the crate.
pub fn get_project_root() -> Result<PathBuf, &'static str> {
    let project_root = env!("PROJECT_ROOT");
    if project_root.is_empty() {
        let error =
            "The crate `prebindgen-project-root` is being used as a regular Cargo dependency.\n\
            Because it is not located within your workspace, it cannot determine the path to the workspace root.\n\
            Please add `prebindgen-project-root` as a member of your workspace and patch your dependencies to use the local path.\n\n\
            You can do this with the helper tool:\n\n\
            cargo install prebindgen-project-root\n\
            cargo prebindgen-project-root install <path>\n\n\
            where `<path>` is the path to your workspace root.\n\n\
            If the patch is already applied and the error persists, verify the version of the patched crate.";
        Err(error)
    } else {
        Ok(project_root.into())
    }
}
