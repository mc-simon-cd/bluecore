# BlueCore Architecture
**License:** Apache 2.0 | **Project:** Simon Project | **Son Güncelleme:** 24 Nisan 2026

---

## Core Philosophy
> "Start lightweight, scale to power engine only when needed."

BlueCore, performans ve esnekliği dengeleyen çift motorlu, modüler bir tarayıcı mimarisi üzerine inşa edilmiştir. Günlük kullanımda OS-native WebView (Tauri) kullanılırken, ağır iş yüklerinde devreye girecek Chromium fork'u hazırda bekler.

---

## 1. Dual-Engine System

| Motor | Varsayılan | Açıklama |
|---|---|---|
| **Tauri Engine** | ✅ | OS-native WebView. Düşük kaynak tüketimi. |
| **Chromium Fork** | ⏸ Standby | Tam uyumluluk ve güçlü render motoru. |

Motor kontrolü `config/engines.json` ve `config/feature_flags.json` üzerinden yapılır.

---

## 2. Core State Architecture

```rust
pub struct BlueCoreState {
    pub engine_manager:     Arc<Mutex<EngineManager>>,
    pub tab_manager:        Arc<Mutex<TabManager>>,
    pub session_manager:    Arc<Mutex<SessionManager>>,
    pub ui_config:          Arc<Mutex<UIConfigManager>>,
    pub split_view_manager: Arc<Mutex<SplitViewManager>>,
}
```

Tüm yöneticiler `tokio::sync::Mutex` ile sarmalanmıştır. Bu sayede Tauri komutları kilit beklerken uygulamanın donması önlenir.

---

## 3. Sistem Yöneticileri

| Yönetici | Sorumluluk |
|---|---|
| `EngineManager` | Motor boot/shutdown/navigate/switch |
| `TabManager` | Sekme oluşturma, kapatma, URL sanitizasyonu |
| `SessionManager` | Kullanıcı oturum yönetimi |
| `UIConfigManager` | Tema, NTP, Vertical Tabs, Reader Mode |
| `SplitViewManager` | Bölünmüş görünüm yönetimi |
| `ModuleRegistry` | Modül ekosistemi (PrivacyShield vb.) |

---

## 4. Communication Bridge ("Structured Bridge")

```
[React Frontend]
      │ invoke(command, args)
      ▼
[BlueCoreAPI wrapper (TypeScript)]
      │
      ▼
[Tauri IPC Layer]
      │
      ▼
[Rust Backend – async Tauri Commands]
      │ .lock().await
      ▼
[Arc<Mutex<Manager>>]
```

- **Request→Response:** Frontend `invoke()` → Rust `#[tauri::command]` → `Result<T, String>`
- **Events:** Rust `app.emit()` → Frontend Event Listener (yükleme yüzdesi, SSL, indirme)

---

## 5. API Kategorileri

| Kategori | Dosya | Komutlar |
|---|---|---|
| Tab & Navigation | `api/tab_api.rs`, `api/navigation_api.rs` | `create_tab`, `close_tab`, `bc_handle_navigation` |
| Engine Control | `api/engine_api.rs` | `switch_engine` |
| UI & Customization | `api/ui_api.rs` | `set_vertical_tabs`, `set_tab_sidebar_collapsed`, `group_tabs_by_domain`, `get_ntp_config`, `set_ntp_background`, `set_browser_theme` |
| Content & Reader | `api/content_api.rs` | `toggle_reader_mode`, `set_reader_preferences`, `start_tts_engine` |
| Split View | `api/split_api.rs` | `enable_split_view`, `update_split_focus`, `resize_split_panes`, `swap_split_panes`, `disable_split_view` |
| Identity | `api/identity_api.rs` | `get_session`, `login`, `logout` |
| Modules | `api/modules.rs` | `get_available_modules`, `execute_module_action` |

---

## 6. Modular System
Modüller `BlueCoreModule` trait'ini implement eder. Direkt motor erişimi yasaktır.

```
/src-tauri/src/modules/
  └── privacy_shield.rs   → BlueCoreModule impl
```

---

## 7. Frontend Structure

```
/src/
  ├── app/layout/   → BrowserShell, TabBar, AddressBar
  ├── hooks/        → useTabs, useModules
  ├── components/   → ModuleCenter (Settings UI)
  └── app/api/      → BlueCoreAPI (Type-safe wrapper)
```
