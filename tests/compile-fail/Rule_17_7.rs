fn func(para1: u16) -> u16 {
    para1
}

fn discarded(para2: u16) {
    func(para2);
    //~^ ERROR Non-compliant - `func` return discarded
}

fn main() {
    discarded(1);
}
