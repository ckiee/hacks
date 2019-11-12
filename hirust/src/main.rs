use std::io;

struct Todo {  
    id: i16,
    title: String,
    completed: bool,
}

fn main() {  
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
		let mut args = input.split_whitespace();
		if input.starts_with("finished") {
			println!("args {:?}", args);
			println!("task# {}", args.next().expect("missing arg"));
			
		}
        println!("input was {:?}", input);
    }
}