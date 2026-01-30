#![no_std]
use tutorlolv2::model::InputGame;

fn main() {
    unsafe {
        tutorlolv2::calculator(core::mem::transmute([0u8; size_of::<InputGame>()]));
        tutorlolv2::realtime(core::mem::transmute(&0));
    }
}
