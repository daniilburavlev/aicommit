use std::process::{Command, exit};

pub fn diff() -> String {
    if let Ok(output) = Command::new("git")
        .args(["diff", "--cached", "--diff-algorithm=minimal"])
        .output()
    {
        if !output.status.success() {
            eprintln!("The current directory must be a Git repository!");
            exit(1);
        }
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        eprintln!("Cannot find git command!");
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_diff() {
        diff();
    }
}
