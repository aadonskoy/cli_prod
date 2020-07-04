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
            1 => {
                println!("Please enter a name for a new product:");
                products.add_product(read_string_input());
            },
            2 => user_search_commands(products),
            4 => user_update_commands(products),
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
            1 => find_by_index(products),
            2 => find_by_text(products),
            3 => break,
            _ => (),
        }

        println!("---------");
    }
}

fn user_update_commands(products: &mut Products) {
    loop {
        println!("Enter id for update:");
        let id = read_number_input();

        if id != 0 {
            println!("Enter new name:");
            match products.update(id, read_string_input()) {
                Ok(()) => {
                    println!("Updated.");
                    break;
                },
                Err(err) => println!("{:?}", err),
            }
        } else {
            println!("Incorrect id.");
        }
    }
}

fn find_by_index(products: &Products) {
    println!("Please enter id:");
    match products.find_by_index(read_number_input()) {
        Some(product) => println!("Found product {:?}", product),
        None => println!("can't find a product with given id"),
    }
}

fn find_by_text(products: &Products) {
    println!("Please enter text:");
    match products.find_by_text(read_string_input()) {
        Some(product) => println!("Found product {:?}", product),
        None => println!("can't find a product with given text in the name"),
    }
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
