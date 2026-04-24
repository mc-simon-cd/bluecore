# BlueCore Browser

> **"Start lightweight, scale to power engine only when needed."**

BlueCore, Rust ve React ile yazılmış yeni nesil, modüler bir tarayıcı projesidir. Tauri'nin hafif WebView motoru varsayılan olarak çalışırken, ihtiyaç duyulduğunda tam kapasiteli bir Chromium fork'una geçiş yapabilir.

---

## ⚖️ Lisans & Proje

| Alan | Bilgi |
|---|---|
| **Lisans** | Apache License 2.0 |
| **Proje** | Simon Project |
| **Durum** | Phase 3 – Modüler API & Kimlik Sistemleri |
| **Son Güncelleme** | 24 Nisan 2026 |

---

## ✨ Özellikler

### 🔵 Çift Motor Sistemi (Dual-Engine)
- **Tauri Engine** (Varsayılan): OS-native WebView, düşük kaynak tüketimi
- **Chromium Fork** (Bekleme): Tam uyumluluk ve güçlü render motoru
- Motorlar arası geçiş hem API hem config üzerinden yapılabilir

### 🗂️ Gelişmiş Sekme Yönetimi
- Dinamik sekme oluşturma, kapatma ve geçiş (`useTabs` hook)
- **Dikey Sekmeler**: Sol panel sekme listesi
- **Sidebar Daraltma**: Sadece favicon görünümü
- **Domain Gruplama**: Aynı siteye ait sekmeleri otomatik grupla

### 🪟 Bölünmüş Görünüm (Split View)
- Tek sekmede iki URL yan yana görüntüleme
- Panel odak yönetimi (araç çubuğu aktif panele bağlanır)
- Gerçek zamanlı panel boyutlandırma (0.2–0.8 oranı)
- Sol/sağ panel yer değiştirme

### 🎨 Özelleştirme (NTP & Tema)
- Yeni Sekme Sayfası (NTP) arka plan ve kısayol yönetimi
- Açık / Koyu / Sistem teması + vurgu rengi desteği
- Kullanıcı tanımlı duvar kağıdı atama

### 📖 Sürükleyici Okuma Modu
- Reklam ve yan panelleri kaldırır
- Gece / Sepya / Varsayılan tema seçenekleri
- Yazı tipi boyutu kişiselleştirme
- Metinden Sese (TTS) entegrasyonu

### 🔐 Kimlik & Oturum Yönetimi
- Güvenli kullanıcı oturum yönetimi (`SessionManager`)
- Şifrelenmiş depolama alanı (`vault_access`)

### 🧩 Modüler Ekosistem
- `BlueCoreModule` trait ile genişletilebilir modül sistemi
- **PrivacyShield**: Tracker ve reklam engelleme (ilk dahili modül)

---

## 🚀 Başlarken

### Gereksinimler
- **Node.js**: Current LTS
- **Rust**: Latest stable
- **Linux Bağımlılıkları**:

  **Arch Linux:**
  ```bash
  sudo pacman -S --needed base-devel libsoup webkit2gtk-4.1 pkgconf
  
  # Symlink geçici çözümü (4.0 yerine 4.1 kullanmak için)
  sudo ln -sf /usr/lib/pkgconfig/webkit2gtk-4.1.pc /usr/lib/pkgconfig/webkit2gtk-4.0.pc
  sudo ln -sf /usr/lib/pkgconfig/javascriptcoregtk-4.1.pc /usr/lib/pkgconfig/javascriptcoregtk-4.0.pc
  sudo ln -sf /usr/lib/libwebkit2gtk-4.1.so /usr/lib/libwebkit2gtk-4.0.so
  ```

  **Debian/Ubuntu:**
  ```bash
  sudo apt-get install -y libsoup2.4-dev libjavascriptcoregtk-4.0-dev libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```

### Çalıştırma

```bash
# Bağımlılıkları yükle
npm install

# Geliştirme modunda çalıştır
npm run tauri dev

# Üretim derlemesi
npm run tauri build
```

---

## 🔬 Deneysel: Chromium Motoru

```typescript
// Runtime'da motor değiştirme
await BlueCoreAPI.engine.switch('chromium');

// Geri geçiş
await BlueCoreAPI.engine.switch('tauri');
```

Config ile aktifleştirme için `docs/enable_chromium.md` dosyasına bakın.

---

## 📁 Dokümantasyon

| Doküman | Açıklama |
|---|---|
| [`docs/architecture.md`](docs/architecture.md) | Sistem mimarisi ve katman yapısı |
| [`docs/engine_switching.md`](docs/engine_switching.md) | Motor değiştirme akışı |
| [`docs/module_system.md`](docs/module_system.md) | Modül sistemi tasarımı |
| [`docs/internal_api.md`](docs/internal_api.md) | Internal API referansı |
| [`docs/enable_chromium.md`](docs/enable_chromium.md) | Chromium aktivasyon kılavuzu |
| [`progress.md`](progress.md) | Proje yol haritası |
| [`tech_debt.md`](tech_debt.md) | Teknik borçlar |

---

© 2026 Simon Project. All rights reserved.
