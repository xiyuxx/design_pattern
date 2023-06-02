// Simple Factory Pattern
// A preface of the design Pattern
// (It's not even one of the 23 design patterns )

// Short describe：
//  receive a parameter,
//  and return different type of product by this parameter's type

// Suppose there is one series of products
// All products have a intro function to introduce itself
// So we can see the intro function as the abstraction of these products and
//   it'll be defined as a trait called show
pub trait Show {
    fn intro(&self);
}

#[derive(Debug)]
pub struct Product {
    name: String,
}

// then we implement trait show for struct Product
impl Show for Product {
    fn intro(&self) {
        println!("Hi! I'm {}", self.name);
    }
}

// The new fn for Product
// In reality, the product may have many many fields,
// but may only need a little fileds to create a useful product
impl Product {
    fn new(name: String) -> Self {
        Product { name }
    }
}


// I use a enum to confine the parameter which will be used by factory
// to create a Product in the create_product function
// it represent the types of product that the factory can create
pub enum ProductType {
    Click,
    Read,
    Write,
}

// Let's define the factory
// The factory has an empty struct block cause it doesn't need to have any fields
// it only has a create_product method,
// which receive a ProductType parameter and then return a Product struct
pub struct SimpleFactory {}

impl SimpleFactory {
    pub fn create_product(pro_type: ProductType) -> Product {
        // return different type of product according to the parameter's type
        match pro_type {
            ProductType::Click => Product::new("click".to_string()),
            ProductType::Read => Product::new("read".to_string()),
            ProductType::Write => Product::new("write".to_string()),
        }
    }
}

#[test]
fn feature() {
    let pro_type = vec![ProductType::Click, ProductType::Read, ProductType::Write];
    let mut pro_vec: Vec<Product> = vec![];
    for pro in pro_type.into_iter() {
        pro_vec.push(SimpleFactory::create_product(pro));
    }

    for product in pro_vec.iter() {
        product.intro();
    }
}
