mod product;
use self::product::*;
fn main() {
    let mut products: Products = Products::new();
    commands_list();
    println!("Your command here:");
    user_main_commands(&mut products);
}

fn user_main_commands(products: &mut Products) {
    loop {
        match read_number_input() {
          1 => products.add_product(),
          2 => user_search_commands(&products),
          5 => products.list_products(),
          3 => {
                println!("Please enter id for deleting product");
                let id = read_number_input();
                if id != 0 {
                    products.delete_by_id(id)
                } else {
                    println!("Isn't a correct id");
                }
            },
          6 => break,
          _ => (),
        };
        println!("---------");
        commands_list();
    }
}

fn user_search_commands(products: &Products) {
    loop {
        println!("Find by:\n1. Index\n2. Text\n3. Exit to main");

        match read_number_input() {
            1 => find_by_index(&products),
            2 => find_by_text(&products),
            3 => break,
            _ => (),
        }

        println!("---------");
    }
}

fn find_by_index(products: &Products) {
    println!("Please enter id:");
    products.find_by_index(read_number_input());
}

fn find_by_text(products: &Products) {
    println!("Please enter text:");
    products.find_by_text(read_string_input());
}

fn read_string_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("can't read input");

    String::from(input.trim_end())
}

fn read_number_input() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("can't read input");

    match input.trim_end().parse::<usize>() {
        Ok(value) => value,
        Err(_) => 0,
    }
}

fn commands_list() {
    println!("1. Add product");
    println!("2. Search product");
    println!("3. Delete product");
    println!("4. Update product");
    println!("5. List products");
    println!("6. Exit");
}
