pub fn simplify_path(path: String) -> String {
    let mut simplified = path.split('/').fold(vec![], |mut stack, p| {
        match p {
            "" | "." => stack,
            ".." => { stack.pop(); stack },
            other => { stack.push(other); stack },
        }
    }).join("/");
    simplified.insert(0, '/');
    simplified
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(simplify_path(String::from("/home/")), String::from("/home"));
        assert_eq!(simplify_path(String::from("/home//foo/")), String::from("/home/foo"));
        assert_eq!(simplify_path(String::from("/../")), String::from("/"));
        assert_eq!(simplify_path(String::from("/a/./b/../../c/")), String::from("/c"));
        assert_eq!(simplify_path(String::from("/a//b////c/d//././/..")), String::from("/a/b/c"));
    }
}
