#![allow(non_snake_case)]
use dioxus::prelude::*;

/// `StateShow` เป็น struct ที่ใช้ในการจัดการสถานะของการแสดง AuthCard
/// ประกอบด้วย field เดียวคือ `show_auth_card` ซึ่งเป็น `Signal<bool>`
/// เพื่อเก็บค่าที่บอกว่าต้องการแสดง AuthCard หรือไม่
#[derive(Clone, PartialEq)]
pub struct StateShow {
    pub show_auth_card: Signal<bool>,
}

impl StateShow {
    /// ฟังก์ชันสร้างใหม่สำหรับ `StateShow`
    /// โดยเริ่มต้นค่า `show_auth_card` เป็น false
    pub fn new() -> Self {
        let show_auth_card = use_signal(|| false);
        Self { show_auth_card }
    }
}