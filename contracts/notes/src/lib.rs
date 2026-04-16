#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec, Address};

// Struktur data escrow
#[contracttype]
#[derive(Clone, Debug)]
pub struct Escrow {
    id: u64,
    client: Address,
    freelancer: Address,
    amount: i128,
    is_released: bool,
}

// Storage key
const ESCROW_DATA: Symbol = symbol_short!("ESCROW");

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {

    // Ambil semua escrow
    pub fn get_escrows(env: Env) -> Vec<Escrow> {
        env.storage().instance().get(&ESCROW_DATA).unwrap_or(Vec::new(&env))
    }

    // 1. Create escrow (client buat + set freelancer + amount)
    pub fn create_escrow(
        env: Env,
        client: Address,
        freelancer: Address,
        amount: i128,
    ) -> String {

        let mut escrows: Vec<Escrow> = env.storage().instance().get(&ESCROW_DATA).unwrap_or(Vec::new(&env));

        let escrow = Escrow {
            id: env.prng().gen::<u64>(),
            client,
            freelancer,
            amount,
            is_released: false,
        };

        escrows.push_back(escrow);

        env.storage().instance().set(&ESCROW_DATA, &escrows);

        String::from_str(&env, "Escrow berhasil dibuat")
    }

    // 2. Release dana ke freelancer
    pub fn release_escrow(env: Env, id: u64) -> String {
        let mut escrows: Vec<Escrow> = env.storage().instance().get(&ESCROW_DATA).unwrap_or(Vec::new(&env));

        for i in 0..escrows.len() {
            let mut escrow = escrows.get(i).unwrap();

            if escrow.id == id {
                if escrow.is_released {
                    return String::from_str(&env, "Dana sudah dicairkan");
                }

                escrow.is_released = true;

                escrows.set(i, escrow);

                env.storage().instance().set(&ESCROW_DATA, &escrows);

                return String::from_str(&env, "Dana berhasil dicairkan ke freelancer");
            }
        }

        String::from_str(&env, "Escrow tidak ditemukan")
    }

    // 3. Cancel escrow (optional, sebelum release)
    pub fn cancel_escrow(env: Env, id: u64) -> String {
        let mut escrows: Vec<Escrow> = env.storage().instance().get(&ESCROW_DATA).unwrap_or(Vec::new(&env));

        for i in 0..escrows.len() {
            let escrow = escrows.get(i).unwrap();

            if escrow.id == id {
                if escrow.is_released {
                    return String::from_str(&env, "Tidak bisa cancel, dana sudah cair");
                }

                escrows.remove(i);

                env.storage().instance().set(&ESCROW_DATA, &escrows);

                return String::from_str(&env, "Escrow berhasil dibatalkan");
            }
        }

        String::from_str(&env, "Escrow tidak ditemukan")
    }
}

mod test;













/* --- CONTOH SCRIPT ---

pub fn get_notes(env: Env) -> Vec<Note> {
    // 1. ambil data notes dari storage
    return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
}

// Fungsi untuk membuat note baru
pub fn create_note(env: Env, title: String, content: String) -> String {
    // 1. ambil data notes dari storage
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object note baru
    let note = Note {
        id: env.prng().gen::<u64>(),
        title: title,
        content: content,
    };
    
    // 3. tambahkan note baru ke notes lama
    notes.push_back(note);
    
    // 4. simpan notes ke storage
    env.storage().instance().set(&NOTE_DATA, &notes);
    
    return String::from_str(&env, "Notes berhasil ditambahkan");
}

// Fungsi untuk menghapus notes berdasarkan id
pub fn delete_note(env: Env, id: u64) -> String {
    // 1. ambil data notes dari storage 
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index note yang akan dihapus menggunakan perulangan
    for i in 0..notes.len() {
        if notes.get(i).unwrap().id == id {
            notes.remove(i);

            env.storage().instance().set(&NOTE_DATA, &notes);
            return String::from_str(&env, "Berhasil hapus notes");
        }
    }

    return String::from_str(&env, "Notes tidak ditemukan")
}


*/