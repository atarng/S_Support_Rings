#![feature(lazy_cell, ptr_sub_ptr)]

use engage::{
    dialog::yesno::BasicDialogItemYes,
    force::*,
    gamedata::{ // Gamedata, JobData, PersonData,
        unit::{Unit, GodUnit,
            UnitEnhanceCalculator, UnitEnhanceFactors, UnitEnhanceValues, 
            UnitRing},
        item::*,
        person::CapabilitySbyte,
        ring::RingData,
        skill::*,
    },
    proc::ProcInst,
    script::*,
    stream::Stream,
};

use skyline::{ install_hook, patching::Patch, };
use std::cmp;
use unity::il2cpp::object::Array;

#[unity::class("App", "UnitRelianceData")]
pub struct UnitRelianceData {
    reliance: u64,
    pub level: i32,
    pub exp: i8,
    pub score: i8,
}

// App.UnitReliance$$TryGet
#[skyline::from_offset(0x01c57860)]
pub fn unitreliance_tryget(pid_a: &Il2CppString, pid_b: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut UnitRelianceData>;

#[unity::from_offset("App", "Transporter", "GetItemCount")]
pub fn transporter_getitemcount(data: &ItemData, method_info: OptionalMethod) -> i32;


// #[unity::from_offset("App", "Unit", "HasItem")]
// 7101a417b0
#[skyline::from_offset(0x01A417B0)]
pub fn unit_hasitem(this: &Unit, item: &ItemData, method_info: OptionalMethod) -> bool;


// Maybe we should make this return the count... in the future.
extern "C" fn install_support_check_script(event: &EventScript) {
    event.register_function("CheckSupportRank", support_try_get);
    event.register_function("CheckPlayerOwnsItem", check_player_owns_item);
}

extern "C" fn check_player_owns_item(args: &Il2CppArray<DynValue>, _method_info: OptionalMethod) -> &'static DynValue {
    let item_data = args.try_get_item(0).unwrap_or_else(|| {
        let iid = args.try_get_string(0).unwrap();
        panic!("CheckPlayerOwnsItem] IID provided ({}) is invalid", iid.to_string());
    });
    
    let mut unit_item_count = 0;
    
    if let Some(absent_units) = Force::get(ForceType::Absent) {
        absent_units.iter().for_each(|unit| unit_item_count += unsafe{ unit_hasitem(unit, item_data, None) } as i32 );
    }
    if let Some(player_units) = Force::get(ForceType::Player) {
        player_units.iter().for_each(|unit| unit_item_count += unsafe{ unit_hasitem(unit, item_data, None) } as i32 );
    }

    let transporter_item_count = unsafe { transporter_getitemcount(item_data, None) };

    let result = (transporter_item_count + unit_item_count) > 0;
    println!("CheckPlayerOwnsItem] transporter_item_count: {0} unit_item_count: {1}",
            transporter_item_count, unit_item_count);

    return DynValue::new_boolean(result);
}

extern "C" fn support_try_get(args: &Il2CppArray<DynValue>, _method_info: OptionalMethod) -> &'static DynValue {
    let pid_a =  args.try_get_string(0);
    let pid_b =  args.try_get_string(1);

    match pid_a {
        Some(_value) => {}
        None => { panic!("support_try_get] pid_a was invalid!"); }
    }
    match pid_b {
        Some(_value) => { println!("support_try_get] {0}", pid_b.unwrap()); }
        None => { panic!("support_try_get] pid_b was invalid!"); }
    }

    let unit_reliance_data = unsafe { unitreliance_tryget(pid_a.unwrap(), pid_b.unwrap(), None) };
    match unit_reliance_data {
        Some(ref _value) => {
            let target_level =  args.try_get_i32(2);
            let unit_reliance_data_unwrapped = unit_reliance_data.unwrap();
            return DynValue::new_boolean(unit_reliance_data_unwrapped.level >= target_level);
        }
        None => {
            panic!("support_try_get] no support data for {0} and {1}", pid_a.unwrap(), pid_b.unwrap());
        }
    }
}

//// HOOKS: END /////

#[skyline::main(name = "Check_Support_Rank")]
pub fn main() {
    let fn_ptr = install_support_check_script as extern "C" fn(&engage::script::EventScript);
    cobapi::install_lua_command_registerer(fn_ptr);
}
