use caprice::Caprice;

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

    loop {
        if let Ok(option) = caprice.eval() {
            if let Some(_) = option {}
        } else {
            break;
        }
    }
}
