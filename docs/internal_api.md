# 🛠️ BlueCore Internal API: Teknik Tasarım ve Stabilizasyon Rehberi

Bu doküman, Phase 3 kapsamında geliştirilen Rust (Backend) ve React (Frontend) arasındaki köprü mekanizmasının mimari standartlarını, iş parçacığı güvenli durum yönetimi prensiplerini ve **2026 modern tarayıcı özelliklerinin** (Özelleştirme, Dikey Sekmeler, Okuma Modu) teknik entegrasyonunu belirler.

## 1. Mimari Yaklaşım: "Structured Bridge"

Tam kapsamlı bir tarayıcıda API, sadece veri gönderip alan bir boru değil, bir **Mesaj Yöneticisidir**.

### A. Tip Güvenlikli İletişim (Type-Safe Dispatcher)
Ham `invoke("command")` çağrıları yerine, TypeScript tarafında tanımlanmış bir `BlueCoreAPI` arayüzü kullanılır. Tüm mesajlar JSON-Schema tabanlı olmalıdır.

### B. Event-Driven Mimari
Rust tarafı, sadece istekleri cevaplamaz; tarayıcı olaylarını (sayfa yüklenme yüzdesi, SSL hataları, indirme durumu) JS tarafına push eder.

---

## 2. Thread-Safe State Management (Durum Yönetimi)

Tarayıcılar doğası gereği çok iş parçacıklı (multi-threaded) yapılardır. BlueCore'un çekirdeği, veri yarışmalarını (data races) önlemek için `Arc<Mutex<T>>` desenini kullanır.

### A. Sistem Yöneticilerinin Güçlendirilmesi
Tüm kritik sistem yöneticileri (TabManager, EngineManager, SessionManager, UIConfigManager), Rust tarafında paylaşılan bir durumda tutulur:

```rust
// Core State Yapısı
pub struct BlueCoreState {
    pub engine_manager: Arc<Mutex<EngineManager>>,
    pub tab_manager: Arc<Mutex<TabManager>>,
    pub session_manager: Arc<Mutex<SessionManager>>,
    pub ui_config: Arc<Mutex<UIConfigManager>>, // Yeni: Özelleştirme ve Düzen yönetimi
}
```

### B. Neden Mutex?
* **Eşzamanlılık:** Frontend'den gelen birden fazla isteğin (örneğin aynı anda iki sekme açma) veriyi bozmasını engeller.
* **Asenkron Kilitleme:** Tauri komutları içinde `tokio::sync::Mutex` kullanılarak, kilit beklenirken tüm uygulamanın donması engellenir.

### C. Deadlock (Ölümcül Kilitlenme) Önleme Stratejisi
1. **Kilit Süresini Kısalt:** Mutex kilidini sadece veriyi okumak veya yazmak için gereken en kısa sürede tutun.
2. **Hiyerarşik Kilitleme:** Eğer birden fazla yönetici kilitlenecekse (örn. hem Tab hem Engine), her zaman aynı sırayla kilitlenmelidir.

---

## 3. API Kategorileri (Core Modules)

### 📂 Navigation & Tab API (Dikey Sekme Odaklı)
* `open_tab(url, engine)`: Belirlenen motor ile yeni sekme açar.
* `set_vertical_tabs(enabled)`: Sekmeleri sol tarafa dikey liste olarak taşır.
* `set_tab_sidebar_collapsed(collapsed)`: Sadece faviconların görüneceği şekilde sütunu daraltır (ekran tasarrufu).
* `group_tabs_by_domain()`: Aynı siteye ait sekmeleri otomatik gruplar ve daraltılabilir/genişletilebilir hale getirir.
* `close_tab(id)`: Sekmeyi kapatır ve kaynakları serbest bırakır.

### 🎨 Customization API (NTP & Tema)
* `get_ntp_config()`: Arka plan (özel resim/preset), kısayol ve tema ayarlarını getirir.
* `manage_ntp_shortcuts(action, site_data)`: Kısayolları ekler, sabitler, yeniden düzenler veya kaldırır.
* `set_ntp_background(source_path)`: Kullanıcı resmi veya sistem duvar kağıdını atar.
* `set_browser_theme(mode, accent_color)`: Açık/koyu mod geçişi ve arka plana göre otomatik renk önerisi sunar.

### 📖 Content & Reader Mode API
* `toggle_reader_mode(id, active)`: Reklam ve yan panelleri temizleyerek sadece metin ve ana görsellere odaklanır.
* `set_reader_preferences(font_size, theme_style)`: Gece modu, sepya veya özel yazı tipi boyutlarını uygular.
* `start_tts_engine(id)`: Metinden Sese (TTS) özelliğini yüksek kaliteli seslendirme ile başlatır.

### 🔐 Identity & Security API
* `get_session()`: Aktif kullanıcı oturum bilgilerini döner.
* `vault_access(key)`: Şifrelenmiş depolama alanına erişim sağlar.

---

## 4. Örnek Uygulama Deseni

### Rust Tarafı (Command Definition)
```rust
#[tauri::command]
pub async fn bc_toggle_vertical_tabs(
    state: State<'_, Arc<Mutex<UIConfigManager>>>,
    enabled: bool
) -> Result<(), String> {
    let mut config = state.lock().await;
    config.update_layout_mode(enabled).await
}
```

### TypeScript Tarafı (Service Wrapper)
```typescript
export const BlueCoreAPI = {
  toggleVerticalTabs: async (enable: boolean) => {
    await invoke('bc_toggle_vertical_tabs', { enabled: enable });
  },
  configureReader: async (settings: ReaderSettings) => {
    await invoke('bc_set_reader_preferences', { settings });
  }
};
```

---

## 5. Stabilizasyon ve Hata Yönetimi Stratejileri

* **Layout Sync:** Dikey sekme moduna geçişte WebView alanı gecikme olmaksızın yeniden boyutlandırılmalıdır.
* **Resource Freezing:** Okuma modu aktifken arka plan scriptleri dondurularak CPU tasarrufu maksimize edilir.
* **NTP Persistence:** Kullanıcı tanımlı kısayollar ve arka planlar SQLite üzerinde asenkron olarak yedeklenir.
* **Circuit Breaker:** Tema yüklemesi hata verirse API otomatik olarak "Default Light" temasına döner.
* **Logging & Audit:** Her API çağrısı `security/audit.log` dosyasına kaydedilir.

---

## 6. Phase 3 İçin Kritik Kontrol Listesi

- [x] Tüm API fonksiyonları için `Result<T, E>` dönüş tipi zorunlu kılındı mı?
- [x] URL girdileri sanitize ediliyor mu? (XSS ve Command Injection önleme)
- [x] `EngineTrait` komutları her iki motor için tutarlı sonuç üretiyor mu?
- [x] Thread-safe state: `tokio::sync::Mutex` ile `Arc<Mutex<T>>` deseni uygulandı mı?
- [ ] Dikey sekmelerde uzun başlıklar için "tooltip" veya tam metin görünümü optimize edildi mi?
- [ ] Okuma Modu'nda görsel medya (video/resim) hiyerarşisi korunuyor mu?
- [ ] NTP kısayolları "En Çok Ziyaret Edilenler" algoritması ile senkronize mi?
- [ ] Tüm `state.lock()` operasyonları için zaman aşımı (timeout) mekanizması eklendi mi?

---
*Son Güncelleme: 24 Nisan 2026*
