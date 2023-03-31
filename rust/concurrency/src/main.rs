mod thread_basic;
mod message_passing;
mod shared_state;

fn main() {
    thread_basic::thread_basic_join();
    thread_basic::thread_basic_move();

    // message_passing::mpsc();
    shared_state::mutex_single();
    shared_state::mutex_multi();
}
