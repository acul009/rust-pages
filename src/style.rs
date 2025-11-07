use std::marker::PhantomData;

pub struct Style<Context> {
    selector: String,
    properties: Vec<(String, String)>,
    context: PhantomData<Context>,
}
