#[derive(Debug)]
pub struct Product {
    pub id: usize,
    pub name: String,
}

type ProductsCollection = Vec<Product>;

#[derive(Debug)]
pub struct Products {
    pub collection: ProductsCollection,
}

impl Products {
    pub fn new() -> Products {
        Products { collection: ProductsCollection::new() }
    }

    pub fn list_products(&self) {
        println!("{:?}", &self.collection);
    }

    pub fn add_product(&mut self) {
        let index = self.collection.len();
        self.collection.push(Product { id: index, name: "test".to_string() });
    }

    pub fn find_by_index(&self, index: usize) {
        let result = &self.collection.iter()
            .find(move |&product| product.id == index);
        match result {
            Some(product) => println!("Found: {:?}", product),
            None => println!("Can't find product"),
        };
    }

    pub fn find_by_text(&self, text: String) {
        let result = &self.collection.iter()
            .find(|&product| product.name.contains(&text));
        match result {
            Some(product) => println!("Found: {:?}", product),
            None => println!("Can't find product"),
        };
    }
}