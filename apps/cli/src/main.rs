use itonda_database::connection;

fn main() {
    println!("Hello from itonda CLI");

    let _pool = connection::connect("sqlite://itonda.db");
}
