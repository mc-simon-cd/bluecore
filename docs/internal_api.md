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

### 📂 Navigation & Tab API
* `open_tab(url, engine)`: Belirlenen motor (Tauri/Chromium) ile yeni sekme açar.
* `set_vertical_tabs(enabled)`: **(2026 Yeni)** Sekme listesini dikey (sol panel) veya yatay mod arasında değiştirir.
* `close_tab(id)`: Sekmeyi kapatır ve kaynakları serbest bırakır.

### 🎨 Customization API (Yeni Sekme & Tema)
* `get_ntp_config()`: Yeni sekme sayfası (NTP) ayarlarını (Arka plan, kısayollar, tema) getirir.
* `update_ntp_setting(key, value)`: Arka plan resmi, kısayol görünürlüğü veya otomatik renk önerilerini günceller.
* `set_browser_theme(mode)`: light, dark veya system temaları arasında geçiş yapar.

### 📖 Content & Reader Mode API
* `toggle_reader_mode(id, active)`: **(2026 Yeni)** Sayfadaki reklam ve kalabalığı temizleyerek "Sürükleyici Okuma Modu"nu başlatır.
* `get_reader_settings()`: Gece modu, sepya, yazı tipi boyutu gibi kişiselleştirme verilerini yönetir.
* `speak_content(id)`: Gelişmiş "Metinden Sese" (TTS) özelliğini kullanarak makaleyi seslendirir.

### 🔐 Identity & Security API
* `get_session()`: Aktif kullanıcı oturum bilgilerini döner.
* `vault_access(key)`: Şifrelenmiş depolama alanına erişim sağlar.

---

## 4. Örnek Uygulama Deseni

### Rust Tarafı (Command Definition)
```rust
#[tauri::command]
pub async fn bc_toggle_reader_mode(
    state: State<'_, Arc<Mutex<UIConfigManager>>>,
    tab_id: String,
    active: bool
) -> Result<ReaderState, String> {
    let mut config = state.lock().await;
    // İçerik temizleme ve DOM manipülasyon sinyalini gönder
    config.set_reader_mode(tab_id, active).await
}
```

### TypeScript Tarafı (Service Wrapper)
```typescript
export const BlueCoreAPI = {
  customizeNTP: async (settings: Partial<NTPSettings>): Promise<void> => {
    await invoke('bc_update_ntp_setting', { settings });
  },
  toggleVerticalView: async (enable: boolean): Promise<void> => {
    await invoke('bc_set_vertical_tabs', { enabled: enable });
  }
};
```

---

## 5. Stabilizasyon ve Hata Yönetimi Stratejileri

* **State Synchronization:** UI tarafındaki dikey sekme görünümü veya tema ayarları, Rust tarafındaki `UIConfigManager` ile her zaman senkronize olmalıdır.
* **Resource Freezing:** Okuma modu aktifken, arka plandaki gereksiz JS betikleri dondurularak CPU tasarrufu sağlanmalıdır.
* **Circuit Breaker:** Eğer bir tema veya arka plan yüklemesi sistem hatasına yol açarsa, API otomatik olarak "Default Light" temasına dönmelidir.
* **Logging & Audit:** Her API çağrısı, güvenlik denetimi için `security/audit.log` dosyasına kaydedilmelidir.

---

## 6. Phase 3 İçin Kritik Kontrol Listesi

- [x] Tüm API fonksiyonları için `Result<T, E>` dönüş tipi zorunlu kılındı mı?
- [x] UI'dan gelen URL girdileri sanitize ediliyor mu? (XSS ve Command Injection önleme)
- [x] `EngineTrait` üzerinden geçen komutlar her iki motor için de tutarlı sonuç üretiyor mu?
- [ ] Yeni Sekme özelleştirmeleri (arka plan/kısayollar) yerel veritabanında (SQLite) kalıcı hale getirildi mi?
- [ ] Dikey Sekmeler aktifken içerik alanının (WebView) boyutu dinamik olarak yeniden hesaplanıyor mu?
- [ ] Sürükleyici Okuma Modu için DOM temizleme algoritması (Sanitization) performansı optimize edildi mi?
- [ ] Tüm `state.lock()` operasyonları için bir zaman aşımı (timeout) mekanizması düşünüldü mü?

---
*Son Güncelleme: 24 Nisan 2026*
