fn main() {
    let a: i32 = 0;
    if (a < 10) && (a > 20) {
        //~^ ERROR Non-compliant - always true
    }
}
