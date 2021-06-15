pub fn simplify(path: &str) -> String {
    let mut items = vec![];

    path.split('/').for_each(|item| match item.trim() {
        "." | "" => println!("item is {}", item),
        ".." | "..." => {
            items.pop();
            println!("item is {}", item)
        }
        _ => items.push(item),
    });

    (["/".to_string(), items.join("/")]).concat()
}
