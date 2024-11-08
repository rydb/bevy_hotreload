pub struct State {
    pub counter: usize,
}

#[no_mangle]
pub fn do_stuff(state: &mut State) {
    state.counter += 2;
    println!("doing stuff in iteration {}", state.counter);
}