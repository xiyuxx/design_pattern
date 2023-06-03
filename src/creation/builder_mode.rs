// Supp...I've written down this word so mant times that
//   I should consider another.
// Imagine you are an inventor, and you have a machine which
//   can produce a wonderful cylinder that
//   can help people do anything(legally).
// When producing a cylinder you must transmit three parameters to your machine,
//   The radius of the bottom, the height and color(we should be facing everyone)
// It receives these parameters sequentially and consequently you get an error
//   if you miss one or even more params.
// And now we can say the builder mode is used in this machine.
// I mean it can build a product part by part, and finally return it

// so the builder struct would be like this
pub struct Builder {
    //these parameters are passed in externally, which means in a given time,
    // some of them may be undefined, and thus use Option to wrap

    // Incidentally, when you become a real inventor
    // these base types may be changed to some complicated structs
    radius: Option<u8>,
    height: Option<f32>,
    color: Option<&'static str>,
}

impl Builder {
    // In this impl block we'll define the setters of every fields
    // every setter would return a mutable reference of the struct
    // so these setters can be called chained

    pub fn radius_setter(&mut self, radius: u8) -> &mut Self {
        self.radius = Some(radius);
        self
    }

    pub fn height_setter(&mut self, height: f32) -> &mut Self {
        self.height = Some(height);
        self
    }

    pub fn color_setter(&mut self, color: &'static str) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn new() -> Self {
        Builder {
            radius: None,
            height: None,
            color: None,
        }
    }
}

impl Builder {
    // In this block the build function will be defined
    // I seperated it from setters just because of my preference and
    //   you can put them together too
    pub fn build(&self) -> Result<Cylinder, &'static str> {
        let radius = self.radius.ok_or("Missing radius")?;
        let height = self.height.ok_or("Missing height")?;
        let color = self.color.ok_or("Missing color")?;
        Ok(Cylinder {
            radius,
            height,
            color,
        })
    }
}

// Let's define our cylinder!
pub struct Cylinder {
    radius: u8,
    height: f32,
    color: &'static str,
}

// I implemented Debug trait manually cause \
//   when I using #[derive(Debug)] there are always some warnings exist
impl std::fmt::Debug for Cylinder{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cylinder")
        .field("radius", &self.radius)
        .field("height", &self.height)
        .field("color", &self.color)
        .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_build() {
        let mut builder = Builder::new();
        let future = builder.radius_setter(5)
        .height_setter(2.5)
        .color_setter("pink")
        .build()
        .unwrap();
        println!("Look what I'm plannin' haha");
        println!("{:#?}",future);
    }

    #[test]
    #[should_panic = "Missing color"]
    fn error_build(){
        let mut builder = Builder::new();
        let future = builder.radius_setter(5)
        .height_setter(2.5)
//        .color_setter("pink")
        .build()
        .unwrap();
        println!("Darkness");
        println!("{:#?}",future);
    }
}
