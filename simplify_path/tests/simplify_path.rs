use simplify_path::simplify;

#[test]
fn works_tail_slash() {
    let input = "/home/";
    assert_eq!(simplify(input), "/home");
}

#[test]
fn works_dot() {
    let input = "/./";
    assert_eq!(simplify(input), "/");
}

#[test]
fn works_double_dot() {
    let input = "/home/../";
    assert_eq!(simplify(input), "/");
}

#[test]
fn works_trible_dot() {
    let input = "/.../";
    assert_eq!(simplify(input), "/");
}

#[test]
fn works_dup_slash() {
    let input = "/home//foo/";
    assert_eq!(simplify(input), "/home/foo");
}

#[test]
fn works_complex() {
    let input = "/a/./b/.../.../c/";
    assert_eq!(simplify(input), "/c");
}
