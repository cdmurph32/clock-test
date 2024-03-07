pub(crate) mod bindings {
    wit_bindgen::generate!(world: "interfaces");
}

use bindings::clocks;

fn main() {
    let datetime = clocks::wall_clock::now();
    println!("{:?}", datetime);
}
