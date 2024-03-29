pub(crate) mod bindings {
    wit_bindgen::generate!("interfaces");
}

use bindings::wasi::clocks;

fn main() {
    let datetime = clocks::wall_clock::now();
    let timezone_display = clocks::timezone::display(datetime);

    println!("{:?}", timezone_display.name);
}
