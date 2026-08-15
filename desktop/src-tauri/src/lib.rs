mod dto;

use dto::{ImuDataDto, NmeaMessageDto};
use serde::Serialize;
use specta::Type;
use tauri::{ipc::Channel, AppHandle, Emitter};
use tauri_specta::{collect_commands, Builder};

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum ReadEvent<'a> {
    UsbFound { path: &'a str },
    Started,
    Progress { progress: i32 },
    Finished,
}
/// A placeholder event example to eventually read data off of a component.
#[tauri::command]
#[specta::specta]
fn read_data(app: AppHandle, on_event: Channel<ReadEvent>) {
    on_event
        .send(ReadEvent::Started)
        .expect("read event to send");

    for progress in [1, 15, 25, 50, 75, 100] {
        on_event
            .send(ReadEvent::Progress { progress })
            .expect("failed to send progress");
    }

    on_event
        .send(ReadEvent::Finished)
        .expect("failed to send finished");
}

/// Commands and types exported to TypeScript.
fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![greet, read_data])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use specta_typescript::Typescript;

    /// Regenerates `desktop/src/bindings.ts`.
    ///
    /// Running this as a test (rather than in `run()`) means CI fails on a
    /// stale checked-in file instead of silently shipping bindings that
    /// disagree with the Rust commands.
    #[test]
    fn export_typescript_bindings() {
        specta_builder()
            .typ::<ReadEvent>()
            .typ::<ImuDataDto>()
            .typ::<NmeaMessageDto>()
            .export(Typescript::default(), "../src/bindings.ts")
            .expect("failed to export typescript bindings");
    }
}
