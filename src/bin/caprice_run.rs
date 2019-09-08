use caprice::Caprice;

fn main() {
    let mut caprice = Caprice::new(functor);

    caprice.init().unwrap();
    loop {
        caprice.parse().unwrap();
    }
}

fn functor(s: String) -> Result<(), std::io::Error> {
    println!("{} activated", s);

    Ok(())
}

