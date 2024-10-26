use color_eyre::Result;
use git2::{DiffOptions, Repository};

pub fn get_changed_files_from_default_branch() -> Result<Vec<String>, git2::Error> {
    // Open the repository
    let repo = Repository::open(match std::env::var("CARGO_WORKSPACE_DIR") {
        Ok(dir) => dir,
        Err(_) => return Err(git2::Error::from_str("CARGO_WORKSPACE_DIR is not set")),
    })?;

    // Get the reference to the default branch (typically 'main' or 'master')
    let default_branch_name = "main"; // Change this if your default branch is not 'main'
    let default_branch_ref = repo.find_reference(&format!("refs/heads/{}", default_branch_name))?;
    let default_branch_commit = default_branch_ref.peel_to_commit()?;

    // Get the reference to the current branch
    let head_ref = repo.head()?;
    let current_commit = head_ref.peel_to_commit()?;

    // Create a diff between the current branch and the default branch
    let mut diff_options = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(
        Some(&default_branch_commit.tree()?),
        Some(&current_commit.tree()?),
        Some(&mut diff_options),
    )?;

    // Collect the paths of changed files from the committed changes
    let mut changed_files = Vec::new();
    diff.print(git2::DiffFormat::NameOnly, |delta, _, _| {
        if let Some(path) = delta.new_file().path() {
            changed_files.push(path.to_string_lossy().into_owned());
        }
        true
    })?;

    // Create a diff for uncommitted changes (working directory vs. current commit)
    let diff_uncommitted = repo.diff_index_to_workdir(None, Some(&mut diff_options))?;

    // Collect the paths of uncommitted changed files
    diff_uncommitted.print(git2::DiffFormat::NameOnly, |delta, _, _| {
        if let Some(path) = delta.new_file().path() {
            changed_files.push(path.to_string_lossy().into_owned());
        }
        true
    })?;

    Ok(changed_files)
}
