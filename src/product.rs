#[derive(Debug)]
pub struct Product {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct UpdateError;

type ProductsCollection = Vec<Product>;

#[derive(Debug)]
pub struct Products {
    collection: ProductsCollection,
    last_index: usize,
}

impl Products {
    pub fn new() -> Products {
        Products { collection: ProductsCollection::new(), last_index: 0 }
    }

    pub fn list_products(&self) {
        println!("{:?}", &self.collection);
    }

    pub fn add_product(&mut self, name: String) {
        let id = self.last_index + 1;
        self.collection.push(Product { id, name });
        self.last_index = id;
    }

    pub fn find_by_index(&self, index: usize) -> Option<&Product> {
        self.collection.iter()
            .find(|&product| product.id == index)
    }

    pub fn find_by_text(&self, text: String) -> Option<&Product> {
        self.collection.iter()
            .find(|&product| product.name.contains(&text))
    }
    pub fn delete_by_id(&mut self, id: usize) {
        println!("Deleting by id: {}", id);
        self.collection.retain(|product| product.id != id);
    }

    pub fn update(&mut self, id: usize, name: String) -> Result<(), UpdateError> {
        match self.collection.iter_mut().find(|product| product.id == id) {
            Some(product) => {
                product.name = name;
                return Ok(());
            },
            None => Err(UpdateError),
        }
    }
}
