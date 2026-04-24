# 🚀 BlueCore Gelişim Yol Haritası (Roadmap)

**Proje:** BlueCore Browser Shell
**Güncel Durum:** Phase 3 (Modüler API ve Kimlik Sistemleri)
**Son Güncelleme:** 24 Nisan 2026

---

## ✅ Tamamlanan Aşamalar

### 🧱 Phase 1: Mimari Temeller
* [x] **Dual-Engine Stratejisi:** Tauri (OS-native) ve Chromium (Standby) yapısı tanımlandı.
* [x] **Backend Başlatma:** Rust tabanlı core, engines ve api klasör yapısı kuruldu.
* [x] **Frontend Çerçevesi:** React tabanlı Browser Shell UI (TabBar, AddressBar) taslakları oluşturuldu.
* [x] **Dokümantasyon:** Mimari ve motor değiştirme protokolleri yazıldı.

### ⚙️ Phase 2: Çekirdek Sistem Entegrasyonu
* [x] **Engine Manager:** Motorların boot/shutdown ve navigasyon mantığı kuruldu.
* [x] **EngineTrait Uygulaması:** Motorlar arası derin soyutlama (Send + Sync) sağlandı.
* [x] **Feature Flag Sistemi:** engines.json üzerinden motor kontrolü aktif edildi.

---

## 🚧 Mevcut Aşama: Phase 3 (Modüler API & Identity)

### 🔵 Tamamlanan ve Devam Eden Görevler
* [x] **BlueCore Internal API:** Rust ve JS arasındaki köprü (Bridge) stabilizasyonu ve 2026 özellik setinin entegrasyonu.
* [x] **Thread-Safe State Management:** Sistem yöneticilerinin `tokio::sync::Mutex` ile güçlendirilmesi.
* [x] **New Tab & Navigation Fix:* Dinamik sekme yönetimi ve navigasyon bridge'i tamamlandı.
* [/] **Identity Layer (Kimlik Katmanı):** Kullanıcı oturum yönetimi ve güvenli depolama entegrasyonu (Devam ediyor).
* [x] **Build System:** Arch Linux uyumluluğu ve modül konsolidasyonu doğrulandı.

### 🎯 Yakındaki Hedefler (Milestones)
* [ ] İlk dahili modülün (Internal Module) çalışma zamanı (runtime) entegrasyonu.
* [ ] Motor değiştirme sırasında "State Migration" (durum taşıma) prototipinin testi.

---

## 🚀 Gelecek Hedefler

### 🧩 Phase 4: Modüler Ekosistem & Güvenlik
* [ ] **Plugin System:** Üçüncü taraf modüller için kısıtlı API erişimi.
* [ ] **AI & Privacy Modules:** Yapay zeka destekli asistan ve reklam engelleyici modülleri.
* [ ] **Security Hardening:** Kum havuzu (Sandboxing) politikalarının sıkılaştırılması.
* [ ] **Network Layer:** Proxy ve VPN desteği için özel ağ katmanı.

### 🌟 Phase 5: Üretim ve Optimizasyon
* [ ] **Chromium Validation:** Chromium fork'un tam kapasite stabilite testleri.
* [ ] **Cross-Platform Support:** Windows ve macOS için özel derleme optimizasyonları.
* [ ] **Beta Launch:** İlk topluluk sürümünün yayınlanması.

---

## ⚠️ Teknik Borçlar ve Notlar
Detaylı takip için [tech_debt.md](file:///home/can/Masaüstü/projeler/bluecore/tech_debt.md) dosyasına bakınız.

* **Performans:** Tauri ve Chromium geçişi arasındaki gecikme (latency) minimize edilmeli.
* **Bellek Yönetimi:** Chromium bekleme modundayken (standby) kaynak tüketimi sıfıra yakın tutulmalı.
* **Güvenlik:** Rust tarafındaki unsafe bloklar periyodik olarak denetlenmeli.

---

## 🧭 Vizyon Notu
BlueCore, düşük kaynak tüketimiyle günlük kullanımda Tauri'yi, ağır iş yüklerinde ise Chromium'u kullanarak tarayıcı dünyasında "dinamik ölçeklenebilirlik" standardını belirlemeyi hedefler.
