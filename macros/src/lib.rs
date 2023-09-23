use proc_macro::*;

#[proc_macro_attribute]
pub fn game_object(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let struc = item.to_string();
    let update = "fn update() {}";

    //if let Some(name) = item.clone().into_iter().nth(2) {
    //    let impl_wrap = format!("{struc} impl GameObject for {name} {{ {update} }}");
    //    return impl_wrap.parse().unwrap();
    //}

    
    item
}

#[proc_macro]
pub fn load_script(item: TokenStream) -> TokenStream {

    let item = item.clone().into_iter().next();

    if let Some(TokenTree::Literal(i)) = item {
        let mut path = "./src/".to_owned();
        path.push_str(&i.to_string());

        let entries = std::fs::read_dir("./").unwrap();

        // Iterate over the directory entries and print the file names.
        for entry in entries {
            let file_name = entry.expect("whuh").file_name();
            println!("{}", file_name.to_string_lossy());
        }


        let file_buf = match std::fs::read(path) {
            Ok(v) => v,
            Err(e) => panic!("Script does not exist."),
        };

        let file = match std::str::from_utf8(&file_buf) {
            Ok(v) => v,
            Err(e) => panic!("Script is not valid UTF 8."),
        };

        return file.parse().unwrap();
    }

    TokenStream::new()
}