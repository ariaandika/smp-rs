use chrono::{DateTime, Utc};



pub struct Presensi {
    pub tanggal: DateTime<Utc>,
    pub nama: String,
    pub kelas: String,
    pub keperluan: String,
    // waktu
}

