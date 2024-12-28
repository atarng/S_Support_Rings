#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use engage::{
    script::*,
};
use cobapi::*;

#[unity::class("App", "UnitRelianceData")]
pub struct UnitRelianceData {
    reliance: u64,
    pub level: i32,
    pub exp: i8,
    pub score: i8,
}
// #[skyline::from_offset(0x01c5a040)]
#[skyline::from_offset(0x01c57860)]
pub fn unit_reliance_try_get(pid_a: &Il2CppString, pid_b: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut UnitRelianceData>;

extern "C" fn support_try_get(args: &Il2CppArray<DynValue>, _method_info: OptionalMethod) -> &'static DynValue {
    let pid_a =  args.try_get_string(0);
    let pid_b =  args.try_get_string(1);

    match pid_a {
        Some(_value) => { println!("support_try_get] {0}", pid_a.unwrap()); }
        None => {
            panic!("support_try_get] pid_a was invalid!");
        }
    }

    match pid_b {
        Some(_value) => { println!("support_try_get] {0}", pid_b.unwrap()); }
        None => {
            panic!("support_try_get] pid_b was invalid!");
        }
    }

    let unit_reliance_data = unsafe { unit_reliance_try_get(pid_a.unwrap(), pid_b.unwrap(), None) };
    match unit_reliance_data {
        Some(ref _value) => {
            let target_level =  args.try_get_i32(2);
            let unit_reliance_data_unwrapped = unit_reliance_data.unwrap();
            println!("support_try_get]  unit_reliance: {0} vs. target_level: {1}", unit_reliance_data_unwrapped.level, target_level);
            return DynValue::new_boolean(unit_reliance_data_unwrapped.level >= target_level);
        }
        None => {
            panic!("support_try_get] no support data for {0} and {1}", pid_a.unwrap(), pid_b.unwrap());
        }
    }
}

#[skyline::main(name = "S_Support_Rings")]
pub fn main() {
    cobapi::install_lua_command_registerer(install_support_check_script);
}

extern "C" fn install_support_check_script(event: &EventScript) {
    event.register_function("CheckSupportRank", support_try_get)
}