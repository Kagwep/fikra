
pub fn parse_config(args: &[String]) ->  &str {
    
    &args[1]

}



// pub fn inspect_args(args: &[String]){

//     if args.len() < 2 {
//         println!("Usage: {} <query> <file_path>", args[0]);
//         return;
//     }

//     let (query, file_path) = parse_config(&args);

//     if query.is_empty() {
//         println!("Error: Query cannot be empty.");
//         return;
//     }

//     if file_path.is_empty() {
//         println!("Error: File path cannot be empty.");
//         return;
//     }

// }