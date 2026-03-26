#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk menyimpan informasi Mahasiswa
#[contracttype]
#[derive(Clone, Debug)]
pub struct Mahasiswa {
    pub id: u64,
    pub nama: String,
    pub jurusan: String,
}

// Storage key untuk data mahasiswa
const MHS_DATA: Symbol = symbol_short!("MHS_DATA");

#[contract]
pub struct MahasiswaContract;

#[contractimpl]
impl MahasiswaContract {
    // Fungsi untuk mendapatkan semua data mahasiswa
    pub fn get_all_mahasiswa(env: Env) -> Vec<Mahasiswa> {
        env.storage()
            .instance()
            .get(&MHS_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk menambah data mahasiswa baru
    pub fn tambah_mahasiswa(env: Env, nama: String, jurusan: String) -> String {
        // 1. Ambil data mahasiswa yang sudah ada
        let mut daftar_mhs: Vec<Mahasiswa> = env.storage()
            .instance()
            .get(&MHS_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Buat objek Mahasiswa baru
        let mahasiswa = Mahasiswa {
            id: env.prng().gen::<u64>(), // Generate ID unik secara acak
            nama,
            jurusan,
        };

        // 3. Masukkan ke dalam daftar
        daftar_mhs.push_back(mahasiswa);

        // 4. Simpan kembali ke storage
        env.storage().instance().set(&MHS_DATA, &daftar_mhs);

        String::from_str(&env, "Data mahasiswa berhasil ditambahkan")
    }

    // Fungsi untuk menghapus mahasiswa berdasarkan ID
    pub fn hapus_mahasiswa(env: Env, id: u64) -> String {
        let mut daftar_mhs: Vec<Mahasiswa> = env.storage()
            .instance()
            .get(&MHS_DATA)
            .unwrap_or(Vec::new(&env));

        // Cari index berdasarkan ID
        for i in 0..daftar_mhs.len() {
            if let Some(mhs) = daftar_mhs.get(i) {
                if mhs.id == id {
                    daftar_mhs.remove(i);
                    env.storage().instance().set(&MHS_DATA, &daftar_mhs);
                    return String::from_str(&env, "Berhasil menghapus data mahasiswa");
                }
            }
        }

        String::from_str(&env, "Data mahasiswa tidak ditemukan")
    }
}

mod test;