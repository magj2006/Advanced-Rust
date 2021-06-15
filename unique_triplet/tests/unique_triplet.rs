use unique_triplet::find;

fn process_vecs(input: &mut Vec<i32>, expect: Vec<[i32; 3]>) {
    let output = find(input);
    assert_eq!(output, expect);
}
#[test]
fn it_works_empty() {
    let expect = vec![];
    let mut input: Vec<i32> = vec![];
    process_vecs(&mut input, expect)
}

#[test]
fn it_works_zero() {
    let expect = vec![];
    let mut input: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0];
    process_vecs(&mut input, expect)
}

#[test]
fn it_works_three() {
    let expect = vec![];
    let mut input: Vec<i32> = vec![-1, 0, 0];
    process_vecs(&mut input, expect)
}

#[test]
fn it_works_one() {
    let expect = vec![];
    let mut input: Vec<i32> = vec![2];
    process_vecs(&mut input, expect)
}

#[test]
fn it_works_four() {
    let expect = vec![[-1, 0, 1]];
    let mut input: Vec<i32> = vec![-1, 0, 1, 1];
    process_vecs(&mut input, expect)
}

#[test]
fn it_works_six() {
    let expect = vec![[-1, -1, 2], [-1, 0, 1]];
    let mut input: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    process_vecs(&mut input, expect)
}

#[test]
fn it_works_nine() {
    let expect = vec![[-1, -1, 2], [-1, 0, 1]];
    let mut input: Vec<i32> = vec![-1, 0, 1, 2, -1, -4, -1, -1, -1];
    process_vecs(&mut input, expect)
}
