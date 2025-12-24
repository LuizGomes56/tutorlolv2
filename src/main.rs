#![no_std]
use tutorlolv2::{model::InputGame, riot::RiotRealtime};

fn main() {
    unsafe {
        tutorlolv2::calculator(core::mem::transmute([0u8; { size_of::<InputGame>() }]));
        tutorlolv2::realtime(&*(0 as *const () as *const RiotRealtime) as &_);
    }
}
