use std::env;

/// PATH environment variable を指定の区切り文字で分割して Vec<String> を返す
///
/// # Arguments
/// * `env_var` - PATH 環境変数の値
///
/// # Returns
/// 分割されたパスの Vec。空または無効なパスは除外
fn parse_paths(env_var: &str) -> Vec<String> {
    env::split_paths(env_var)
        .map(|p| p.to_string_lossy().trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

fn main() {
    match env::var("PATH") {
        Ok(path_var) => {
            let paths = parse_paths(&path_var);
            for path in paths {
                println!("{}", path);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to read PATH environment variable: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::join_paths;

    use super::*;

    #[test]
    fn test_parse_paths_single_path() {
        let result = parse_paths("/usr/bin");
        assert_eq!(result, vec!["/usr/bin"]);
    }

    #[test]
    fn test_parse_paths_multiple_paths() {
        let paths = join_paths(["/usr/bin", "/usr/local/bin", "/home/user/bin"])
            .unwrap()
            .into_string()
            .unwrap();
        let result = parse_paths(&paths);
        assert_eq!(result, vec!["/usr/bin", "/usr/local/bin", "/home/user/bin"]);
    }

    #[test]
    fn test_parse_paths_empty_string() {
        let result = parse_paths("");
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_parse_paths_only_separators() {
        let paths = join_paths(["", "", "", ""]).unwrap().into_string().unwrap();
        let result = parse_paths(&paths);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_parse_paths_trailing_separator() {
        let paths = join_paths(["/usr/bin", "/usr/local/bin", ""])
            .unwrap()
            .into_string()
            .unwrap();
        let result = parse_paths(&paths);
        assert_eq!(result, vec!["/usr/bin", "/usr/local/bin"]);
    }

    #[test]
    fn test_parse_paths_leading_separator() {
        let paths = join_paths(["", "/usr/bin", "/usr/local/bin"])
            .unwrap()
            .into_string()
            .unwrap();
        let result = parse_paths(&paths);
        assert_eq!(result, vec!["/usr/bin", "/usr/local/bin"]);
    }

    #[test]
    fn test_parse_paths_with_spaces() {
        let paths = join_paths([" /usr/bin ", " /usr/local/bin "])
            .unwrap()
            .into_string()
            .unwrap();
        let result = parse_paths(&paths);
        assert_eq!(result, vec!["/usr/bin", "/usr/local/bin"]);
    }
}
