use std::thread;

fn main() {
    let game_server_thread = thread::spawn(|| {
        game_server::main().unwrap();
    });

    let sdk_server_thread = thread::spawn(|| {
        sdk_server::main();
    });

    game_server_thread.join().unwrap();
    sdk_server_thread.join().unwrap();
}
