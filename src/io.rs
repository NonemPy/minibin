extern crate gpw;
extern crate linked_hash_map;
extern crate owning_ref;
extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use linked_hash_map::LinkedHashMap;

use owning_ref::RwLockReadGuardRef;

use std::cell::RefCell;
use std::env;
use std::sync::{PoisonError, RwLock};

lazy_static! {
    static ref ENTRIES: RwLock<LinkedHashMap<String, String>> = RwLock::new(LinkedHashMap::new());
    static ref BUFFER_SIZE: usize = env::var("PASTEBIN_MAX_PASTES")
        .map(|f| f
            .parse::<usize>()
            .expect("Failed to parse value of PASTEBIN_MAX_PASTES"))
        .unwrap_or(1000usize);
}

/// Ensures `ENTRIES` is less than the size of `PASTEBIN_MAX_PASTES`. If it isn't then
/// `ENTRIES.len() - PASTEBIN_MAX_PASTES` elements will be popped off the front of the map.
///
/// During the purge, `ENTRIES` is locked and the current thread will block.
fn purge_old() {
    let entries_len = ENTRIES.read().unwrap_or_else(PoisonError::into_inner).len();

    if entries_len > *BUFFER_SIZE {
        let to_remove = entries_len - *BUFFER_SIZE;

        let mut entries = ENTRIES.write().unwrap_or_else(PoisonError::into_inner);

        for _ in 0..to_remove {
            entries.pop_front();
        }
    }
}

/// Generates a 'pronounceable' random ID using gpw
pub fn generate_id() -> String {
    thread_local!(static KEYGEN: RefCell<gpw::PasswordGenerator> = RefCell::new(gpw::PasswordGenerator::default()));
    KEYGEN.with(|k| k.borrow_mut().next()).unwrap_or_else(|| {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .collect::<String>()
    })
}

/// Stores a paste under the given id
pub fn store_paste(id: String, content: String) {
    purge_old();
    ENTRIES
        .write()
        .unwrap_or_else(PoisonError::into_inner)
        .insert(id, content);
}

/// Get a paste by id.
///
/// Returns `None` if the paste doesn't exist.
pub fn get_paste(id: &str) -> Option<RwLockReadGuardRef<LinkedHashMap<String, String>, String>> {
    let or = RwLockReadGuardRef::new(ENTRIES.read().unwrap_or_else(PoisonError::into_inner));

    if or.contains_key(id) {
        Some(or.map(|x| x.get(id).unwrap()))
    } else {
        None
    }
}
