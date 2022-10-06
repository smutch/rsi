use std::path::Path;
use eyre::Result;

/// Parse a job submission script and return the sbatch arguments
/// # Arguments
///
/// * `script` - The path to a batch submission script with `#SBATCH ...` arguments
fn parse_script(script: &Path) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(script)?;
    let mut result: Vec<String> = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("#SBATCH") {
            result.extend(line.split_whitespace().skip(1).map(|s| s.to_string()));
        }
    }
    Ok(result)
}

/// Extract job submission parameters from a batch script and use that to estimate the start time
/// of the job.
///
/// # Arguments
///
/// * `script` - The path to a batch submission script with `#SBATCH ...` arguments
pub fn starttime(script: &Path) -> Result<()> {
    unimplemented!()
}


#[cfg(test)]
mod tests {

    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn can_parse_script_correctly() {
        let dir = tempdir().expect("Failed to create temporary directory!");
        let script = dir.path().join("submit.sh");
        write(&script, r#"
              #SBATCH --job-name=hello
              #SBATCH --nodes=1
              #SBATCH --ntasks-per-node=1
              #SBATCH --time=00:01:00
              #SBATCH -c 1
              # This is just a comment and should not be parsed.
              # #SBATCH This is a red herring too!
              #SBATCH --partition=debug

              srun hostname
              "#
             ).expect("Failed to write contents of temp file!");
        let args = super::parse_script(&script).expect("Failed to parse script!");

        assert_eq!(args[0], "--job-name=hello");
        assert_eq!(args[1], "--nodes=1");
        assert_eq!(args[2], "--ntasks-per-node=1");
        assert_eq!(args[3], "--time=00:01:00");
        assert_eq!(args[4..=5], ["-c", "1"]);
        assert_eq!(args[6], "--partition=debug");
    }

}
