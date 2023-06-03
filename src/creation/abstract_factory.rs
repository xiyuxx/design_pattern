// In the previous two(simple_factory,factory_method) design pattern,
// One factory can only create one type of product...too weak!too weak!
// Abstration Factory allows factory to create one product family,
// which means the label,logo...? like Apple Factory.
// I'll use Apple below.

// Let's define some more specific products, I choose Phone, Watch
pub trait Phone {
    // To be simple,this phone can only be used to call
    fn call(&self);
}

pub struct IPhone {}
impl Phone for IPhone{
    fn call(&self) {
        println!("Apple Phone pro max ultra is calling!");
    }
}

pub struct MiPhone {}
impl Phone for MiPhone{
    fn call(&self) {
        println!("Mi Phone 6 is calling!");
    }
}

pub trait Watch {
    // Check time it's the first function that comes to my mind
    fn check_time(&self);
}

pub struct IWatch{}
impl Watch for IWatch{
    fn check_time(&self) {
        println!("Check time by IWatch! Made in heaven!");
    }
}
pub struct MiWatch{}
impl Watch for MiWatch{
    fn check_time(&self) {
        println!("Check time by MiWatch! The World!");
    }
}

// Let's think about the factory definition
// Each factory can create two products here,
// So there's two methods in trait
pub trait Factory {
    fn create_phone(&self)->Box<dyn Phone>;
    fn create_watch(&self)->Box<dyn Watch>;
}

// Impl this trait for two concrete factory
pub struct AppleFactory{}
impl Factory for AppleFactory{
    fn create_phone(&self)->Box<dyn Phone> {
        Box::new(IPhone{})
    }
    fn create_watch(&self)->Box<dyn Watch> {
        Box::new(IWatch{})
    }
}
pub struct MiFactory{}
impl Factory for MiFactory{
    fn create_phone(&self)->Box<dyn Phone> {
        Box::new(MiPhone{})
    }
    fn create_watch(&self)->Box<dyn Watch> {
        Box::new(MiWatch{})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Factory;
    
    #[test]
    fn test_name() {
        let apple = AppleFactory{};
        let mi = MiFactory{};
        let mut phone:Vec<Box<dyn Phone>> = vec![];
        phone.push(apple.create_phone());
        phone.push(mi.create_phone());
        
        let mut watch:Vec<Box<dyn Watch>> = vec![];
        watch.push(apple.create_watch());
        watch.push(mi.create_watch());
        
        for i in 0..2{
            phone[i].call();
            watch[i].check_time();
            println!();
        }
    }
}
