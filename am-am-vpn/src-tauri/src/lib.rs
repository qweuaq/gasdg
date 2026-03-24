pub mod commands;
pub mod core;
pub mod models;
pub mod proxy;
pub mod state;
pub mod storage;
pub mod subscription;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("am-am-vpn");

    let app_state = AppState::new(data_dir);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Subscription
            commands::subscription::add_subscription,
            commands::subscription::refresh_subscription,
            commands::subscription::remove_subscription,
            commands::subscription::list_subscriptions,
            // Servers
            commands::server::list_servers,
            commands::server::test_latency,
            commands::server::test_all_latencies,
            commands::server::select_fastest_server,
            // Connection
            commands::connection::connect,
            commands::connection::disconnect,
            commands::connection::get_connection_state,
            commands::connection::get_settings,
            commands::connection::update_settings,
            commands::connection::get_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Am-Am VPN");
}
