use std::sync::Mutex;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    AppHandle, Emitter, Manager,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_updater::UpdaterExt;

struct AppState {
    pinned: bool,
    dark_mode: bool,
}

fn build_menu(app: &AppHandle, pinned: bool, dark_mode: bool) -> tauri::Result<Menu<tauri::Wry>> {
    let version = app.package_info().version.to_string();

    let pin_item = CheckMenuItem::with_id(app, "pin", "항상 위에 표시", true, pinned, None::<&str>)?;
    let dark_item = MenuItem::with_id(
        app,
        "dark",
        if dark_mode { "라이트 모드" } else { "다크 모드" },
        true,
        None::<&str>,
    )?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let sep3 = PredefinedMenuItem::separator(app)?;
    let sep4 = PredefinedMenuItem::separator(app)?;
    let refresh_item = MenuItem::with_id(app, "refresh", "새로고침", true, Some("CmdOrCtrl+R"))?;
    let devtools_item = MenuItem::with_id(app, "devtools", "개발자 도구", true, Some("CmdOrCtrl+Alt+I"))?;
    let update_item = MenuItem::with_id(app, "update", "업데이트 확인", true, None::<&str>)?;
    let version_item = MenuItem::with_id(app, "version", format!("v{}", version), false, None::<&str>)?;

    let submenu = Submenu::with_id_and_items(
        app,
        "app_menu",
        "뭐먹었니",
        true,
        &[
            &pin_item,
            &sep1,
            &dark_item,
            &sep2,
            &refresh_item,
            &sep3,
            &devtools_item,
            &sep4,
            &update_item,
            &version_item,
        ],
    )?;

    Menu::with_items(app, &[&submenu])
}

fn rebuild_menu(app: &AppHandle, pinned: bool, dark_mode: bool) -> tauri::Result<()> {
    let menu = build_menu(app, pinned, dark_mode)?;
    app.set_menu(menu)?;
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState { pinned: false, dark_mode: false }));

            let menu = build_menu(app.handle(), false, false)?;
            app.set_menu(menu)?;

            let app_handle = app.handle().clone();
            app.on_menu_event(move |app, event| {
                let state_mutex = app.state::<Mutex<AppState>>();

                match event.id().as_ref() {
                    "pin" => {
                        let (pinned, dark) = {
                            let mut s = state_mutex.lock().unwrap();
                            s.pinned = !s.pinned;
                            (s.pinned, s.dark_mode)
                        };
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.set_always_on_top(pinned);
                        }
                        let _ = rebuild_menu(app, pinned, dark);
                    }
                    "dark" => {
                        let (pinned, dark) = {
                            let mut s = state_mutex.lock().unwrap();
                            s.dark_mode = !s.dark_mode;
                            (s.pinned, s.dark_mode)
                        };
                        if let Some(win) = app.get_webview_window("main") {
                            let theme = if dark { "dark" } else { "light" };
                            let _ = win.emit("theme-change", theme);
                        }
                        let _ = rebuild_menu(app, pinned, dark);
                    }
                    "refresh" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.eval("window.location.reload()");
                        }
                    }
                    "devtools" => {
                        if let Some(win) = app.get_webview_window("main") {
                            if win.is_devtools_open() {
                                win.close_devtools();
                            } else {
                                win.open_devtools();
                            }
                        }
                    }
                    "update" => {
                        let handle = app_handle.clone();
                        tauri::async_runtime::spawn(async move {
                            check_update(handle).await;
                        });
                    }
                    _ => {}
                }
            });

            // 패키징된 앱에서만 자동 업데이트 확인
            #[cfg(not(debug_assertions))]
            {
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    check_update(handle).await;
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn check_update(app: AppHandle) {
    let updater = match app.updater() {
        Ok(u) => u,
        Err(_) => return,
    };
    let Ok(Some(update)) = updater.check().await else { return };

    app.dialog()
        .message("새 버전이 있습니다. 백그라운드에서 다운로드합니다.")
        .title("업데이트 확인")
        .blocking_show();

    if update.download_and_install(|_, _| {}, || {}).await.is_ok() {
        let restart = app.dialog()
            .message("업데이트가 준비됐습니다. 재시작 후 적용됩니다.")
            .title("업데이트 준비 완료")
            .blocking_show();
        if restart {
            app.restart();
        }
    }
}
