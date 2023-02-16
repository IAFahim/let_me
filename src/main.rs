#[derive(Debug)]
struct Component {
    id: u32,
    name: String,
    input: String,
    on_enable: Option<Box<Component>>,
    start: Task,
    result: Option<Box<Component>>,
    on_complete: Option<Box<Component>>,
    on_error: Option<Box<Component>>,
    next: Option<Box<Component>>,
}

#[derive(Debug)]
struct Task{
    lib: String,
    method: String,
}

fn main(){
    let mut component = Component{
        id: 1,
        name: "test".to_string(),
        input: "test".to_string(),
        on_enable: None,
        start: Task{
            lib: "test".to_string(),
            method: "test".to_string(),
        },
        result: None,
        on_complete: None,
        on_error: None,
        next: None,
    };
    let mut component2 = Component{
        id: 2,
        name: "test".to_string(),
        input: "test".to_string(),
        on_enable: Some(Box::new(component)),
        start: Task{
            lib: "test".to_string(),
            method: "test".to_string(),
        },
        result: None,
        on_complete: None,
        on_error: None,
        next: None,
    };
    println!("{:?}", component2);
}