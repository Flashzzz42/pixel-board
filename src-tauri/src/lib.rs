use tauri::Manager;

// 界面整体缩放（前端按钮调用）：设置 WebView2 ZoomFactor，效果同浏览器页面缩放。
// WebView2 专属 API，macOS/iOS（WKWebView）无 ZoomFactor，按平台门控。
#[cfg(target_os = "windows")]
#[tauri::command]
fn set_zoom(app: tauri::AppHandle, factor: f64) -> Result<(), String> {
    let win = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    let f = factor.clamp(0.5, 2.0);
    win.with_webview(move |webview| {
        if let Err(e) = unsafe { webview.controller().SetZoomFactor(f) } {
            eprintln!("failed to set zoom: {e}");
        }
    })
    .map_err(|e| e.to_string())
}

fn build() -> tauri::Builder<tauri::Wry> {
    let builder = tauri::Builder::default();
    #[cfg(target_os = "windows")]
    let builder = builder.invoke_handler(tauri::generate_handler![set_zoom]);
    builder
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    build()
        .setup(|app| {
            // 关键：WebView2 用户数据目录默认落在临时位置，localStorage 每次重启会丢。
            // 这里指到 %LOCALAPPDATA%\com.pixelboard.app，草稿/设置持久保存。
            let data_dir = app.path().app_local_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let win = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
            .title("像素工坊")
            .inner_size(1360.0, 860.0)
            .min_inner_size(900.0, 600.0)
            .center()
            .background_color(tauri::window::Color(245, 246, 250, 255))
            .data_directory(data_dir)
            .build()?;
            #[cfg(target_os = "windows")]
            setup_download_handler(&win)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 导出 PNG / 草稿文件走 <a download>，WebView2 嵌入式默认不一定弹保存框。
// 接管 DownloadStarting：按建议文件名扩展名分流（.json 草稿文件 / 其他视为 PNG），
// 弹原生「另存为」对话框，用户确认后落盘；取消则取消下载。
#[cfg(target_os = "windows")]
fn setup_download_handler(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    use webview2_com::DownloadStartingEventHandler;
    use windows::core::HSTRING;
    use windows::core::PWSTR;

    use windows::core::Interface;

    window.with_webview(|webview| {
        let core = unsafe {
            webview
                .controller()
                .CoreWebView2()
                .expect("failed to get WebView2 core")
        };
        // add_DownloadStarting 定义在 ICoreWebView2_4（下载事件属于该版本接口），需 QI 上去。
        let core: webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_4 = core
            .cast()
            .expect("failed to QI ICoreWebView2_4");
        let handler = DownloadStartingEventHandler::create(Box::new(move |_, args| {
            let Some(args) = args else {
                return Ok(());
            };
            // 读取浏览器建议的下载路径（含 <a download> 指定的文件名），按扩展名选保存框。
            // 注：webview2-com 的 ICoreWebView2DownloadOperation 绑定了却缺 SuggestedFileName，
            // 这里改用 DownloadStartingEventArgs 的 ResultFilePath（建议路径末尾即文件名）。
            let mut result_path = PWSTR::null();
            let suggested = unsafe {
                if args.ResultFilePath(&mut result_path).is_ok() && !result_path.is_null() {
                    String::from_utf16_lossy(result_path.as_wide())
                } else {
                    String::new()
                }
            };
            let is_json = suggested.to_lowercase().ends_with(".json");
            let file_name = suggested
                .rsplit(|c| c == '/' || c == '\\')
                .next()
                .unwrap_or("")
                .to_string();
            let default_name = if file_name.is_empty() {
                if is_json { "草稿.json".to_string() } else { "pixel-board.png".to_string() }
            } else {
                file_name
            };
            let path = if is_json {
                rfd::FileDialog::new()
                    .add_filter("画板草稿文件", &["json"])
                    .set_file_name(default_name)
                    .save_file()
            } else {
                rfd::FileDialog::new()
                    .add_filter("PNG 图片", &["png"])
                    .set_file_name(default_name)
                    .save_file()
            };
            if let Some(path) = path {
                unsafe {
                    args.SetResultFilePath(&HSTRING::from(path.to_string_lossy().to_string()))?;
                    args.SetHandled(true)?;
                }
            } else {
                unsafe {
                    args.SetCancel(true)?;
                    args.SetHandled(true)?;
                }
            }
            Ok(())
        }));
        let mut token = 0i64;
        if let Err(e) = unsafe { core.add_DownloadStarting(&handler, &mut token) } {
            eprintln!("failed to register DownloadStarting handler: {e}");
        }
    })
}
