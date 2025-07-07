use teus_config::config;
use teus_database::storage;
use teus_api::routes;
use teus_monitor::sys::SysInfo;
use std::{
    env,
    path::Path,
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

fn main() {
    println!("Starting Teus service...");
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 {
        let path = &args[1];
        // Check if the provided file exists.
        if !Path::new(path).exists() {
            eprintln!("Configuration file '{}' does not exist.", path);
            process::exit(1);
        }
        path.clone()
    } else {
        // Use default config path if none is provided.
        "./teus-dev.toml".to_string()
    };

    let config = match config::parser::load_config(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            return;
        }
    };

    // Initialize Storage once
    let storage = match storage::Storage::new(&config.database.path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize storage: {}", e);
            process::exit(1); // Exit if storage initialization fails
        }
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Handle Ctrl+C (SIGINT)
    ctrlc::set_handler(move || {
        println!("Signal received, stopping...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set Ctrl+C handler");

    // Start the webserver in a separate thread
    let config_clone_for_web = config.clone(); // Clone config for webserver
    let storage_clone_for_web = storage.clone(); // Clone storage for webserver
    let web_handle_thread = thread::spawn(move || {
        /* TODO: consider a RC instead to avoid cloning */
        let _ = routes::start_webserver(&config_clone_for_web, storage_clone_for_web);
    });

    // Give the webserver a moment to start
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("Teus service started");

    // Run the system monitor in the main thread
    // The monitor will create its own Storage instance as needed per run_monitor cycle for now.
    // This could be further refactored if SysInfo instance became long-lived.
    while running.load(Ordering::SeqCst) {
        let sysinfo = SysInfo::default();
        sysinfo.run_monitor(&config); // run_monitor internally creates Storage for its DB ops

        thread::sleep(std::time::Duration::from_secs(config.monitor.interval_secs));
    }

    // Wait for the webserver to finish and ensure a clean shutdown
    println!("Closing, waiting for other threads to finish...");
    if let Err(e) = web_handle_thread.join() {
        eprintln!("Error waiting for webserver thread: {:?}", e);
    }
}
