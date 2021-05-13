mod front {
    pub mod hosting {
        pub fn add() {
            println!("front::hosting::add");
        }
    }
}

mod ka {
    pub fn fc() {
        use front::hosting::add;
        add();
    }
}

fn main() {
    use ka::fc;
    fc();
}
