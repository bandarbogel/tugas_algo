use std::io;

// Definisikan struktur data untuk menyimpan informasi barang
#[derive(Debug)]
struct Barang {
    nama: String,
    harga: f64,
    jumlah: u32,
}

fn main() {
    // Buat vektor untuk menyimpan barang-barang
    let mut daftar_barang: Vec<Barang> = Vec::new();

    loop {
        println!("Pilih operasi:");
        println!("1. Tambah barang");
        println!("2. Lihat daftar barang");
        println!("3. Keluar");

        let mut opsi = String::new();
        io::stdin().read_line(&mut opsi).expect("Gagal membaca baris");

        let opsi: u32 = match opsi.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input tidak valid. Masukkan nomor yang valid.");
                continue;
            }
        };

        match opsi {
            1 => tambah_barang(&mut daftar_barang),
            2 => lihat_daftar_barang(&daftar_barang),
            3 => {
                println!("Keluar dari program.");
                break;
            }
            _ => {
                println!("Opsi tidak valid. Masukkan nomor yang valid.");
            }
        }
    }
}

fn tambah_barang(daftar_barang: &mut Vec<Barang>) {
    println!("Masukkan nama barang:");
    let mut nama = String::new();
    io::stdin().read_line(&mut nama).expect("Gagal membaca baris");

    let nama = nama.trim().to_string();

    println!("Masukkan harga barang:");
    let mut harga = String::new();
    io::stdin().read_line(&mut harga).expect("Gagal membaca baris");

    let harga: f64 = match harga.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input tidak valid. Masukkan angka yang valid.");
            return;
        }
    };

    println!("Masukkan jumlah barang:");
    let mut jumlah = String::new();
    io::stdin().read_line(&mut jumlah).expect("Gagal membaca baris");

    let jumlah: u32 = match jumlah.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input tidak valid. Masukkan angka yang valid.");
            return;
        }
    };

    let barang = Barang { nama, harga, jumlah };
    daftar_barang.push(barang);

    println!("Barang berhasil ditambahkan!");
}

fn lihat_daftar_barang(daftar_barang: &Vec<Barang>) {
    println!("Daftar Barang:");
    for barang in daftar_barang {
        println!("{:?}", barang);
    }
}
