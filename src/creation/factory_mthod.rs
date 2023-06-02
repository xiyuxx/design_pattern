// In the simple factory, we defined a trait which is implemented by products
// In factory method, we'll do it also, but one thing different is
//  that we'll define a trait for factory struct!
// The product_create method will be defined in trait
// And if you want a factory struct, just impl the trait for your struct

// The Product trait
trait Show {
    fn intro(&self);
}

struct Product {
    name: String,
}

impl Show for Product {
    fn intro(&self) {
        println!("This is a {}", self.name);
    }
}
impl Product {
    fn new(name: String) -> Product {
        Product { name }
    }
}

// The trait for factory
trait Factory {
    // return a struct which has implemented the trait Show with Box wrapped
    fn create_product() -> Box<dyn Show>;
}

// define a TVFactory
// it can store some integral fields for product creating
// it can receive parameters to create different product
// but in here I just make it empty
struct TVFactory {}
impl Factory for TVFactory {
    fn create_product() -> Box<dyn Show> {
        Box::new(Product::new("TV".to_string()))
    }
}

// There is no air conditioner in my dormitory
// I have to endure 30+ degrees in Wuhan
struct AirConditionFactory {}
impl Factory for AirConditionFactory {
    fn create_product() -> Box<dyn Show> {
        Box::new(Product::new("AirCondition".to_string()))
    }
}

#[test]
fn feature() {
    let tv = TVFactory::create_product();
    let air = AirConditionFactory::create_product();
    tv.intro();
    air.intro();
}
