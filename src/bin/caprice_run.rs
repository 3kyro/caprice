use caprice::{Caprice, CapriceCommand};
use std::time::{Instant, Duration};
fn main() {
    let mut caprice = Caprice::new()
        .set_prompt("!:")
        .enable_raw_screen()
        // .enable_alternate_screen()
        .init();

    // caprice.set_callback(|x| println!("{}", x.len()));

    caprice.set_keywords(&vec![
        "some_token".to_owned(),
        "some_o".to_owned(),
        "some_ot".to_owned(),
        "some_oth".to_owned(),
        "some_othe".to_owned(),
        "some_other".to_owned(),
        "some_other_t".to_owned(),
        "some_other_tok".to_owned(),
        "some_other_toke".to_owned(),
        "some_other_token".to_owned(),
        "none".to_owned(),
    ]);

    let (tx, rx) = caprice.run();

    


    let now = Instant::now();
    loop {
        if let Ok(token) = rx.try_recv() {
            println!("got {}", token);
        }

        if now.elapsed() > Duration::from_secs(5) {
            if tx.send(CapriceCommand::Exit).is_ok() {
                std::thread::sleep(Duration::from_secs(1));
                println!("bye!");
                break;
            } else {
                println!("you already left");
                break;
            }
        }
    }

    

}
