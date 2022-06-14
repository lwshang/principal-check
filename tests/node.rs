#![cfg(target_arch = "wasm32")]
#![allow(unused)]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

#[repr(packed)]
pub struct PrincipalInner {
    // This is also the structure of new Principal
    len: u8,
    bytes: [u8; 29],
}

struct PrincipalOld(PrincipalInner);

pub struct PrincipalNoPacked {
    len: u8,
    bytes: [u8; 29],
}

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(std::mem::size_of::<PrincipalInner>(), 30);
    assert_eq!(std::mem::size_of::<PrincipalOld>(), 30);
    assert_eq!(std::mem::size_of::<PrincipalNoPacked>(), 30);
}