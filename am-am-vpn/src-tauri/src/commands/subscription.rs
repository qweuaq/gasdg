//! Tauri IPC commands for subscription management.

use tauri::State;
use crate::state::AppState;
use crate::models::Subscription;
use crate::subscription::parser;

#[tauri::command]
pub async fn add_subscription(
    url: String,
    state: State<'_, AppState>,
) -> Result<Subscription, String> {
    let sub = parser::fetch_subscription(&url).await?;
    let mut subs = state.subscriptions.lock().await;
    subs.push(sub.clone());
    state.persist_subscriptions(&subs)?;
    Ok(sub)
}

#[tauri::command]
pub async fn refresh_subscription(
    id: String,
    state: State<'_, AppState>,
) -> Result<Subscription, String> {
    let mut subs = state.subscriptions.lock().await;
    let idx = subs.iter().position(|s| s.id == id)
        .ok_or("Subscription not found")?;
    let url = subs[idx].url.clone();
    let refreshed = parser::fetch_subscription(&url).await?;
    let refreshed = Subscription { id: id.clone(), ..refreshed };
    subs[idx] = refreshed.clone();
    state.persist_subscriptions(&subs)?;
    Ok(refreshed)
}

#[tauri::command]
pub async fn remove_subscription(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut subs = state.subscriptions.lock().await;
    subs.retain(|s| s.id != id);
    state.persist_subscriptions(&subs)?;
    Ok(())
}

#[tauri::command]
pub async fn list_subscriptions(
    state: State<'_, AppState>,
) -> Result<Vec<Subscription>, String> {
    let subs = state.subscriptions.lock().await;
    Ok(subs.clone())
}
