# 🚀 BlueCore Gelişim Yol Haritası (Roadmap)

**Proje:** BlueCore Browser Shell
**Güncel Durum:** Phase 3+ (Modüler API, 2026 Özellikleri)
**Son Güncelleme:** 24 Nisan 2026

---

## ✅ Tamamlanan Aşamalar

### 🧱 Phase 1: Mimari Temeller
* [x] **Dual-Engine Stratejisi:** Tauri (OS-native) ve Chromium (Standby) yapısı tanımlandı.
* [x] **Backend Başlatma:** Rust tabanlı core, engines ve api klasör yapısı kuruldu.
* [x] **Frontend Çerçevesi:** React tabanlı Browser Shell UI (TabBar, AddressBar) taslakları oluşturuldu.
* [x] **Dokümantasyon:** Mimari ve motor değiştirme protokolleri yazıldı.
* [x] **Lisans:** Apache 2.0'dan MPL-2.0'a geçiş yapıldı.

### ⚙️ Phase 2: Çekirdek Sistem Entegrasyonu
* [x] **Engine Manager:** Motorların boot/shutdown/navigate ve switch mantığı kuruldu.
* [x] **EngineTrait Uygulaması:** `Result<T, E>` döndüren, motorlar arası derin soyutlama (Send + Sync) sağlandı.
* [x] **Feature Flag Sistemi:** `engines.json` ve `feature_flags.json` üzerinden motor kontrolü etkinleştirildi.

### 🔧 Phase 3: Modüler API, İş Parçacığı Güvenliği & 2026 Özellikleri

#### API Köprüsü & Hata Yönetimi
* [x] **Internal API Stabilizasyonu:** Tüm core API'leri `Result<T, String>` dönüş tipiyle standartlaştırıldı.
* [x] **URL Sanitizasyonu:** `TabManager` içinde XSS/CI koruması, boş URL engeli ve protokol zorunluluğu uygulandı.
* [x] **BlueCoreAPI (TS):** Tip güvenli TypeScript wrapper ile frontend-backend köprüsü kuruldu.

#### Thread-Safe Durum Yönetimi
* [x] **tokio::sync::Mutex:** Tüm yöneticiler `std::sync::Mutex`'ten asenkron kilide geçirildi.
* [x] **BlueCoreState:** `EngineManager`, `TabManager`, `SessionManager`, `UIConfigManager`, `SplitViewManager` tek merkezi yapıda toplandı.
* [x] **Arc sarmalama:** Tüm yöneticiler güvenli paylaşım için `Arc<Mutex<T>>` ile sarmalandı.

#### Sekme & Navigasyon Sistemi
* [x] **Dinamik sekme yönetimi:** `useTabs` hook ile frontend tab state'i backend ile senkronize edildi.
* [x] **Navigation API Bridge:** `bc_handle_navigation` komutu ile tab URL güncelleme ve motor tetikleme kuruldu.
* [x] **TabManager genişletmesi:** `update_tab_url()` metodu eklendi.

#### UIConfigManager & 2026 Özellikleri
* [x] **UIConfigManager:** Tema, NTP, Vertical Tabs, Reader Mode, Tab Grupları yönetimi.
* [x] **Dikey Sekmeler:** `set_vertical_tabs`, `set_tab_sidebar_collapsed`.
* [x] **Tab Gruplama:** `group_tabs_by_domain` — domain'e göre otomatik gruplama.
* [x] **NTP Özelleştirme:** `get_ntp_config`, `update_ntp_setting`, `set_ntp_background`.
* [x] **Tema Yönetimi:** `set_browser_theme` — Light/Dark/System + vurgu rengi.
* [x] **Reader Mode:** `toggle_reader_mode`, `set_reader_preferences`.
* [x] **TTS Engine:** `start_tts_engine` — Metinden sese (placeholder).

#### SplitViewManager (Yeni)
* [x] **Bölünmüş Görünüm:** `enable_split_view` — tek sekmede iki URL yan yana.
* [x] **Panel Odağı:** `update_split_focus` — "left" / "right" aktif panel.
* [x] **Boyutlandırma:** `resize_split_panes` — 0.2–0.8 arası oran ile gerçek zamanlı.
* [x] **Panel Takas:** `swap_split_panes` — sol/sağ yer değiştirme.
* [x] **Kapatma & Sorgulama:** `disable_split_view`, `get_split_view_state`.

#### Kimlik Katmanı
* [x] **SessionManager:** `get_session`, `login`, `logout` komutları.
* [/] **Vault (Güvenli Depolama):** `vault_access` implementasyonu devam ediyor.

#### Modül Sistemi
* [x] **ModuleRegistry:** `RwLock` ile thread-safe modül kaydı ve aksiyon çalıştırma.
* [x] **PrivacyShield:** İlk dahili modül — tracker ve reklam engelleme.

#### Dokümantasyon
* [x] **internal_api.md:** Teknik tasarım ve stabilizasyon rehberi.
* [x] **architecture.md, engine_switching.md, module_system.md:** Tam yeniden yazım.
* [x] **README.md:** Phase 3+ özellik seti ve kurulum kılavuzu güncellendi.
* [x] **tech_debt.md:** Teknik borçlar takip edilmeye başlandı.

---

## 🚧 Devam Edenler & Yakındaki Hedefler

* [/] **Vault Güvenli Depolama:** `vault_access` için şifreli SQLite entegrasyonu.
* [ ] **NTP SQLite Kalıcılığı:** Kısayollar ve arka plan verilerini uygulama yeniden açıldığında koruma.
* [ ] **TTS Entegrasyonu:** OS-native (Speech-dispatcher/SAPI) veya Cloud TTS bağlantısı.
* [ ] **State Migration:** Bölünmüş görünüm kapatıldığında pasif panel scroll/form verisi korunacak.
* [ ] **Keyboard Shortcuts:** `Shift+Alt+N` / `Cmd+Option+N` split view kısayolları.
* [ ] **PrivacyShield Runtime Testi:** Gerçek URL filtreleme ve engelleme testi.

---

## 🚀 Gelecek Hedefler

### 🧩 Phase 4: Modüler Ekosistem & Güvenlik
* [ ] **Plugin System:** Üçüncü taraf modüller için kısıtlı API erişimi.
* [ ] **AI Assistant Module:** Yapay zeka destekli sayfa özetleme ve asistan.
* [ ] **Security Hardening:** Kum havuzu (Sandboxing) politikalarının sıkılaştırılması.
* [ ] **Network Layer:** Proxy ve VPN desteği için özel ağ katmanı.
* [ ] **Audit Log:** `security/audit.log` ile API çağrısı denetimi.

### 🌟 Phase 5: Üretim ve Optimizasyon
* [ ] **Chromium Validation:** Chromium fork'un tam kapasite stabilite testleri.
* [ ] **Cross-Platform Support:** Windows ve macOS için özel derleme optimizasyonları.
* [ ] **Performance Tuning:** Tauri ↔ Chromium geçiş gecikmesi minimizasyonu.
* [ ] **Beta Launch:** İlk topluluk sürümünün yayınlanması.

---

## ⚠️ Teknik Borçlar
Detaylı takip için [tech_debt.md](tech_debt.md) dosyasına bakınız.

* **NTP Kalıcılığı:** SQLite entegrasyonu eksik (yüksek öncelik).
* **TTS:** OS-native entegrasyon placeholder aşamasında.
* **Motor Geçiş Gecikmesi:** Tauri ↔ Chromium arası State Migration optimizasyonu.
* **Timeout Mekanizması:** `state.lock()` operasyonları için zaman aşımı henüz uygulanmadı.

---

## 🧭 Vizyon Notu
BlueCore, düşük kaynak tüketimiyle günlük kullanımda Tauri'yi, ağır iş yüklerinde ise Chromium'u kullanarak tarayıcı dünyasında **"dinamik ölçeklenebilirlik"** standardını belirlemeyi hedefler. 2026 yılında eklenen Dikey Sekmeler, Bölünmüş Görünüm ve Sürükleyici Okuma Modu ile modern tarayıcı deneyimini yeniden tanımlamaktadır.
