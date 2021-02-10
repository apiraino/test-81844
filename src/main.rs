use serde_derive::Deserialize;

struct AB {}

impl<'de> serde::Deserialize<'de> for AB {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)] // comment this out to fix the stack overflow
        struct A2 {}

        return Ok(AB {});
    }
}

fn main() {
    println!("Hello, world!");
}
