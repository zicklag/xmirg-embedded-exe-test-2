fn main() {
    println!("Starting xmrig");
    // Starts xmrig and returns a handle that can be used to kill the process, wait for the process
    // to finish, etc.
    let mut xmrig = xmrig::start_xmrig().expect("Could not start xmrig");

    // Waits 3 seconds
    std::thread::sleep(std::time::Duration::from_secs(3));

    // And then kills the process
    println!("Killing xmrig");
    xmrig.process.kill().unwrap();
}
