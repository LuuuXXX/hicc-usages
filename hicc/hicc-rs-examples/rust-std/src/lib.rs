#![feature(specialization)]

use hicc_rs::export_lib;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ptr::NonNull;
use std::sync::{Mutex, OnceLock, RwLock};

// ===== core: NonNull =====

fn nonnull_string() -> NonNull<String> {
    let s = Box::into_raw(Box::new(String::from("hello")));
    unsafe { NonNull::new_unchecked(s) }
}

// ===== core: Option =====

fn new_option_i32() -> Option<i32> {
    Some(42)
}

fn new_option_string() -> Option<String> {
    Some(String::from("hello"))
}

fn new_option_none_i32() -> Option<i32> {
    None
}

// ===== core: Result =====

fn new_result_ok_i32() -> Result<i32, bool> {
    Ok(100)
}

fn new_result_err_bool() -> Result<i32, bool> {
    Err(false)
}

// ===== core: Array =====

fn new_array_option_i32() -> [Option<i32>; 3] {
    [Some(10), None, Some(30)]
}

// ===== core: Tuple =====

fn new_tuple_i32_i32() -> (i32, i32) {
    (1, 2)
}

fn new_tuple_i32_string() -> (i32, String) {
    (99, String::from("world"))
}

fn new_tuple_i32_i32_i32() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn new_tuple_i32_i32_i32_i32() -> (i32, i32, i32, i32) {
    (1, 2, 3, 4)
}

fn new_tuple_i32_i32_i32_i32_i32() -> (i32, i32, i32, i32, i32) {
    (1, 2, 3, 4, 5)
}

fn new_tuple_i32_i32_i32_i32_i32_i32() -> (i32, i32, i32, i32, i32, i32) {
    (1, 2, 3, 4, 5, 6)
}

// ===== core: Slice &str =====

fn new_str() -> &'static str {
    "hello"
}

// ===== core: Cell =====

fn new_cell_i32() -> Cell<i32> {
    Cell::new(42)
}

// ===== core: RefCell =====

fn new_refcell_i32() -> RefCell<i32> {
    RefCell::new(42)
}

// ===== alloc: Vec =====

fn new_vec_i32() -> Vec<i32> {
    Vec::new()
}

fn new_vec_string() -> Vec<String> {
    Vec::new()
}

// ===== alloc: String =====

fn new_string() -> String {
    String::from("hello")
}

fn new_string_empty() -> String {
    String::new()
}

// ===== alloc: Box =====

fn new_box_i32() -> Box<i32> {
    Box::new(42)
}

fn new_box_string() -> Box<String> {
    Box::new(String::from("hello"))
}

// ===== alloc: Rc =====

fn new_rc_i32() -> Rc<i32> {
    Rc::new(42)
}

use alloc_crate::boxed::Box;
use alloc_crate::rc::Rc;
use alloc_crate::sync::Arc;
extern crate alloc as alloc_crate;

// ===== alloc: Arc =====

fn new_arc_i32() -> Arc<i32> {
    Arc::new(42)
}

// ===== alloc: BTreeMap =====

fn new_btreemap_i32_i32() -> BTreeMap<i32, i32> {
    BTreeMap::new()
}

fn new_btreemap_string_string() -> BTreeMap<String, String> {
    BTreeMap::new()
}

// ===== alloc: BTreeSet =====

fn new_btreeset_i32() -> BTreeSet<i32> {
    BTreeSet::new()
}

fn new_btreeset_string() -> BTreeSet<String> {
    BTreeSet::new()
}

// ===== std: HashMap =====

fn new_hashmap_i32_i32() -> HashMap<i32, i32> {
    HashMap::new()
}

fn new_hashmap_string_string() -> HashMap<String, String> {
    HashMap::new()
}

// ===== std: HashSet =====

fn new_hashset_i32() -> HashSet<i32> {
    HashSet::new()
}

fn new_hashset_string() -> HashSet<String> {
    HashSet::new()
}

// ===== std: OnceLock =====

fn new_oncelock_i32() -> OnceLock<i32> {
    OnceLock::new()
}

// ===== std: Mutex =====

fn new_mutex_i32() -> Mutex<i32> {
    Mutex::new(42)
}

// ===== std: RwLock =====

fn new_rwlock_i32() -> RwLock<i32> {
    RwLock::new(42)
}

#[export_lib(name = "rust_std")]
mod ffi {
    use super::*;

    // core: NonNull
    fn nonnull_string() -> NonNull<String>;

    // core: Option
    fn new_option_i32() -> Option<i32>;
    fn new_option_string() -> Option<String>;
    fn new_option_none_i32() -> Option<i32>;

    // core: Result
    fn new_result_ok_i32() -> Result<i32, bool>;
    fn new_result_err_bool() -> Result<i32, bool>;

    // core: Array
    fn new_array_option_i32() -> [Option<i32>; 3];

    // core: Tuple
    fn new_tuple_i32_i32() -> (i32, i32);
    fn new_tuple_i32_string() -> (i32, String);
    fn new_tuple_i32_i32_i32() -> (i32, i32, i32);
    fn new_tuple_i32_i32_i32_i32() -> (i32, i32, i32, i32);
    fn new_tuple_i32_i32_i32_i32_i32() -> (i32, i32, i32, i32, i32);
    fn new_tuple_i32_i32_i32_i32_i32_i32() -> (i32, i32, i32, i32, i32, i32);

    // core: Slice &str
    fn new_str() -> &'static str;

    // core: Cell
    fn new_cell_i32() -> Cell<i32>;

    // core: RefCell
    fn new_refcell_i32() -> RefCell<i32>;

    // alloc: Vec
    fn new_vec_i32() -> Vec<i32>;
    fn new_vec_string() -> Vec<String>;

    // alloc: String
    fn new_string() -> String;
    fn new_string_empty() -> String;

    // alloc: Box
    fn new_box_i32() -> Box<i32>;
    fn new_box_string() -> Box<String>;

    // alloc: Rc
    fn new_rc_i32() -> Rc<i32>;

    // alloc: Arc
    fn new_arc_i32() -> Arc<i32>;

    // alloc: BTreeMap
    fn new_btreemap_i32_i32() -> BTreeMap<i32, i32>;
    fn new_btreemap_string_string() -> BTreeMap<String, String>;

    // alloc: BTreeSet
    fn new_btreeset_i32() -> BTreeSet<i32>;
    fn new_btreeset_string() -> BTreeSet<String>;

    // std: HashMap
    fn new_hashmap_i32_i32() -> HashMap<i32, i32>;
    fn new_hashmap_string_string() -> HashMap<String, String>;

    // std: HashSet
    fn new_hashset_i32() -> HashSet<i32>;
    fn new_hashset_string() -> HashSet<String>;

    // std: OnceLock
    fn new_oncelock_i32() -> OnceLock<i32>;

    // std: Mutex
    fn new_mutex_i32() -> Mutex<i32>;

    // std: RwLock
    fn new_rwlock_i32() -> RwLock<i32>;
}
