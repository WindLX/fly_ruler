use stick::{Event, Listener};

#[tokio::main]
async fn main() {
    let lis = Listener::default();
    let mut controller_option = Some(lis.await);

    loop {
        if let Some(controller) = &mut controller_option {
            match controller.await {
                Event::Exit(id) => {
                    println!("{} exit", id);
                    controller_option = None;
                }
                e => {
                    println!("event {:?}", e);
                }
            }
        } else {
            break;
        }
    }
}
