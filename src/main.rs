#[derive(Debug)]
struct Product {
    id: usize,
    name: String,
}

type ProductsCollection = Vec<Product>;

#[derive(Debug)]
struct Products {
    collection: ProductsCollection,
}

impl Products {
    fn new() -> Products {
        Products { collection: ProductsCollection::new() }
    }

    fn list_products(&self) {
        println!("{:?}", &self.collection);
    }

    fn add_product(&mut self) {
        &self.collection.push(Product { id: 1, name: "test".to_string() });
    }
}

fn main() {
    let mut products: Products = Products::new();
    commands_list();
    println!("Your command here:");
    user_action(&mut products);
}

fn user_action(mut products: &mut Products) {
    loop {
        match read_input() {
          1 => products.add_product(),
          5 => products.list_products(),
          6 => break,
          _ => commands_list(),
        };
    }
}

fn read_input() -> usize {
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
