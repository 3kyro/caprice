use caprice::Caprice;

fn main() {
    let mut caprice = Caprice::new(functor);

    caprice.set_tokens(&vec![
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

    caprice.init().unwrap();
    loop {
        caprice.parse().unwrap();
    }
}

fn functor(s: String) -> Result<(), std::io::Error> {
    println!("{} activated", s);

    Ok(())
}

