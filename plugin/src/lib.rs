#![feature(lazy_cell, ptr_sub_ptr)]

// use cobapi::*;
use engage::{
    // menu::{BasicMenu, BasicMenuItem},
    // dialog::yesno::BasicDialogItemYes,
    force::*,
    gamedata::{ // Gamedata, JobData, PersonData,
        unit::{Unit, // GodUnit,
            UnitEnhanceCalculator, UnitEnhanceFactors, UnitEnhanceValues, 
            UnitRing},
        item::*,
        person::CapabilitySbyte,
        ring::RingData,
        skill::*,
    },
    // singleton::SingletonClass,
    script::*,
    stream::Stream,
};

use skyline::{ install_hook, patching::Patch, };
use std::cmp;
// use unity::prelude::*;
use unity::il2cpp::object::Array;


#[unity::class("App", "UnitRelianceData")]
pub struct UnitRelianceData {
    reliance: u64,
    pub level: i32,
    pub exp: i8,
    pub score: i8,
}

#[unity::class("App", "InfoUtil_StatusSkill")]
pub struct StatusSkill {
    pub skill_data: Option<&'static SkillData>,
    pub is_active: bool,
    pub category: i32
}

// App.UnitReliance$$TryGet
#[skyline::from_offset(0x01c57860)]
pub fn unitreliance_tryget(pid_a: &Il2CppString, pid_b: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut UnitRelianceData>;

#[unity::from_offset("App", "Transporter", "GetItemCount")]
pub fn transporter_getitemcount(data: &ItemData, method_info: OptionalMethod) -> i32;

// 0x7101fc72e0
//App_SkillData_o * App.InfoUtil.StatusSkill$$get_Data(App_InfoUtil_StatusSkill_o *__this,MethodInfo *method)
#[skyline::from_offset(0x01fc72e0)]
pub fn infoutil_statusskill_getdata(this: &StatusSkill, _method_info: u64) -> Option<&SkillData>;

// 0x7101FC7300
// bool App.InfoUtil.StatusSkill$$get_IsActive(App_InfoUtil_StatusSkill_o *__this,MethodInfo *method)
#[skyline::from_offset(0x1FC7300)]
pub fn infoutil_statusskill_getisactive(this: &StatusSkill, _method_info: u64) -> bool;

// 0x7101FC7320
// int32_t App.InfoUtil.StatusSkill$$get_Category (App_InfoUtil_StatusSkill_o *__this,MethodInfo *method)
#[skyline::from_offset(0x1FC7320)]
pub fn infoutil_statusskill_getcategory(this: &StatusSkill, _method_info: u64) -> i32;

// 0x7101FC7310
// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_IsActive")]
#[skyline::from_offset(0x1FC7310)]
pub fn infoutil_statusskill_setisactive(this: &StatusSkill, active: bool, _method_info: u64);

// 0x7101FC72F0
// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_Data")]
#[skyline::from_offset(0x1FC72F0)]
pub fn infoutiil_statusskill_setdata(this: &StatusSkill, value: Option<&SkillData>, _method_info: u64);

// 0x7101FC7330
// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_Category")]
#[skyline::from_offset(0x1FC7330)]
pub fn infoutil_statusskill_setcategory(this: &StatusSkill, cat: i32, _method_info: u64);

// 0x71024246b0
// App_CapabilitySbyte_o * App.RingData$$get_Enhance(App_RingData_o *__this,MethodInfo *method)
#[skyline::from_offset(0x024246b0)]
pub fn ringdata_getenhance(this: &RingData, _method_info: u64) -> Option<&CapabilitySbyte>;

// 0x7101a54ee0
// App_SkillArray_o * App.Unit$$get_EquipSkill(App_Unit_o *__this,MethodInfo *method)
#[skyline::from_offset(0x01a54ee0)]
pub fn unit_getequipskill(this: &Unit, _method_info: u64) -> Option<&SkillArray>;

// #[unity::from_offset("App", "Unit", "HasItem")]
// 7101a417b0
#[skyline::from_offset(0x01A417B0)]
pub fn unit_hasitem(this: &Unit, item: &ItemData, method_info: OptionalMethod) -> bool;


// 0x7101a2bcd0
// int32_t App.Unit$$GetMaxHp(App_Unit_o *__this,MethodInfo *method)
#[skyline::from_offset(0x01a2bcd0)]
pub fn unit_getmaxhp(this: &Unit, _method_info: u64) -> i32;

#[skyline::from_offset(0x01f7bd40)]
pub fn unitenhancevalues_add2(this: &UnitEnhanceValues, capability: Option<&CapabilitySbyte>, is_not_enhance: bool, _method_info : u64);

#[skyline::from_offset(0x01f781f0)]
pub fn unitenhancecalculator_addimpl(this: &UnitEnhanceCalculator, capability: Option<&CapabilitySbyte>, is_not_enhance: bool, _method_info : u64);

// 0x7102dec860
// int8_t App.CapabilityBase<sbyte>$$get_Hp(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DEC860 *method)
#[skyline::from_offset(0x02dec860)]
pub fn capabilitybase_sbyte_gethp(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102dec8c0
// int8_t App.CapabilityBase<sbyte>$$get_Str(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DEC8C0 *method)
#[skyline::from_offset(0x02dec8c0)]
pub fn capabilitybase_sbyte_getstr(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102decaa0
// int8_t App.CapabilityBase<sbyte>$$get_Magic(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DECAA0 *method)
#[skyline::from_offset(0x02decaa0)]
pub fn capabilitybase_sbyte_getmag(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102dec920
// int8_t App.CapabilityBase<sbyte>$$get_Tech(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DEC920 *method)
#[skyline::from_offset(0x02dec920)]
pub fn capabilitybase_sbyte_getskl(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102dec980
// int8_t App.CapabilityBase<sbyte>$$get_Quick(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DEC980 *method)
#[skyline::from_offset(0x02dec980)]
pub fn capabilitybase_sbyte_getspd(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102deca40
// int8_t App.CapabilityBase<sbyte>$$get_Def(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DECA40 *method)
#[skyline::from_offset(0x02deca40)]
pub fn capabilitybase_sbyte_getdef(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102decb00
// int8_t App.CapabilityBase<sbyte>$$get_Mdef(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DECB00 *method)
#[skyline::from_offset(0x02decb00)]
pub fn capabilitybase_sbyte_getres(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102dec9e0
// int8_t App.CapabilityBase<sbyte>$$get_Luck(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DEC9E0 *method)
#[skyline::from_offset(0x02dec9e0)]
pub fn capabilitybase_sbyte_getlck(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102decb60
// int8_t App.CapabilityBase<sbyte>$$get_Phys(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DECB60 *method)
#[skyline::from_offset(0x02decb60)]
pub fn capabilitybase_sbyte_getbld(this: &CapabilitySbyte, method: u64) -> i8;

// 0x7102decc20
// int8_t App.CapabilityBase<sbyte>$$get_Move(App_CapabilityBase_sbyte__o *__this,MethodInfo_2DECC20 *method)
#[skyline::from_offset(0x02decc20)]
pub fn capabilitybase_sbyte_getmov(this: &CapabilitySbyte, method: u64) -> i8;

//// EXTERN-BEG: defined for lua /////

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

//// EXTERN-END /////

//// HOOKS: BEG /////

// 0x7102911030
// App_InfoUtil_StatusSkill_array * ::::: &'static mut Array<&'static StatusSkill>
#[unity::hook("App", "InfoUtil", "GetSkillListForUnitInfo")]
pub fn infoutil_getskilllistforunitinfo(unit: Option<&Unit>, is_equip: bool, is_pack: bool, mut size: i32, _method_info : u64) ->
        &'static mut Array<Option<&'static StatusSkill>>
{unsafe{
    let mut ring_skills_to_add : usize = 0;
    if let Some(person) = unit {
        if let Some(unit_ring) = person.get_ring() {
            let ring_skills = unit_ring.fields.data.get_equip_skills();
            for i in 0..ring_skills.len() {
                let skill_array_entity = &ring_skills[i];
                if let Some(e_skill) = skill_array_entity.get_skill() {
                    let sid = e_skill.sid.to_string();
                    let empty = sid == "SID_無し" || sid == "無し" || sid == "";
                    if !empty { ring_skills_to_add += 1; }
                }
            }
        }
    }

    // For now, assume only 1 bond ring skill
    size = cmp::max(size, 10) + ring_skills_to_add as i32;
    let original: &'static mut Array<Option<&'static StatusSkill>> = call_original!(unit, is_equip, is_pack, size, _method_info);
    if ring_skills_to_add == 0 {
        return original;
    }

    if let Some(person) = unit {
    if let Some(unit_ring) = person.get_ring() {
    if let Some(god) = person.get_god_unit() {
        let ring_data = unit_ring.fields.data;
        let ring_skills = ring_data.get_equip_skills();
        let mut inheritance_skills_present = 0;
        let mut ring_skills_present = 0;

        // ring_skills length always 32.
        if let Some(equip_skills) = unit_getequipskill(person, _method_info) {
            // SkillArray
            let mut start = 0;
            for i in 0..original.len() {
                if let Some(skill_exists) = original[i] {
                    let category = infoutil_statusskill_getcategory(skill_exists, _method_info);
                    if category == 6 { ring_skills_present += 1; }
                    if category == 11 { inheritance_skills_present += 1; }
                }
            }

            // Count how many inherited skills we have.
            let mut inheritance_skills_in_equip_list = 0;
            let mut ring_skills_in_equip_list = 0;
            for i in 0..equip_skills.len() {
                let skill_array_entity = &equip_skills[i];
                if let Some(_e_skill) = skill_array_entity.get_skill() {
                    let skill_category = skill_array_entity.get_category();
                    if skill_category == 11 { inheritance_skills_in_equip_list += 1; }
                    if skill_category == 6 { ring_skills_in_equip_list += 1; }
                }
            }

            let mut jobskill_present : bool = false;
            let mut ring_start_index = 0;
            if !is_equip {
                // 0: personal
                // 1: Job(?)
                // 2: Inheritance
                // 3: Inheritance
                if let Some(candidate_job_skill) = original[1] {
                    jobskill_present = infoutil_statusskill_getcategory(candidate_job_skill, _method_info) == 2 &&
                        infoutil_statusskill_getisactive(candidate_job_skill, _method_info);
                }
                start = if jobskill_present { 3 } else { 2 };
                ring_start_index = if jobskill_present { 2 } else { 1 };
            }
                
            let slots_needed = (2 - inheritance_skills_present) + (ring_skills_in_equip_list - ring_skills_present);
            // Shift everything over by number of skills that need to be added
            for i in (start..original.len()).rev() {
                let index_to_source = if i < slots_needed { 0 } else { i - slots_needed };
                original[i] = original[index_to_source as usize];
            }

            if inheritance_skills_present < 2 {
                // Fill with empty slots
                let inheritance_start_index = start + if is_equip { inheritance_skills_present } else { 0 };
                let inheritance_fin_index   = inheritance_start_index + (2 - inheritance_skills_present);
                for i in inheritance_start_index..inheritance_fin_index {
                    let dupet = Il2CppClass::from_name("App", "InfoUtil").unwrap().get_nested_types().iter().find(|t| t.get_name() == "StatusSkill").unwrap();
                    let newt: &'static StatusSkill = il2cpp::instantiate_class::<StatusSkill>(dupet).unwrap();
                    original[i as usize] = Some(newt);
                    if let Some(original_i_unwrapped) = original[i as usize] {
                        let skill_category  = 11;
                        infoutil_statusskill_setcategory(original_i_unwrapped, skill_category, _method_info); 
                        infoutil_statusskill_setisactive(original_i_unwrapped, false, _method_info);
                        infoutiil_statusskill_setdata(original_i_unwrapped, None, _method_info);
                    } else {
                        panic!("[infoutil_getskilllistforunitinfo] No StatusSkill at index: {}", i);
                    }
                }
                ring_start_index = inheritance_fin_index;
            }
            if ring_skills_present < ring_skills_in_equip_list {
                let skill_category = 6;
                let ring_fin_index = ring_start_index + (ring_skills_in_equip_list - ring_skills_present);
                for i in ring_start_index..ring_fin_index {
                    let dupet = Il2CppClass::from_name("App", "InfoUtil").unwrap().get_nested_types().iter().find(|t| t.get_name() == "StatusSkill").unwrap();
                    let newt: &'static StatusSkill = il2cpp::instantiate_class::<StatusSkill>(dupet).unwrap();
                    original[i as usize] = Some(newt);
                    let ring_skills_index = ring_skills_present + (i - ring_start_index);
                    if let Some(original_i_unwrapped) = original[i as usize] {
                        let skill_array_entity = &ring_skills[ring_skills_index];
                        if let Some(r_skill) = skill_array_entity.get_skill() {
                            infoutil_statusskill_setcategory(original_i_unwrapped, skill_category, _method_info); 
                            infoutiil_statusskill_setdata(original_i_unwrapped, Some(r_skill), _method_info);
                            infoutil_statusskill_setisactive(original_i_unwrapped, true, _method_info);
                        }
                    }
                }
            }
        }
    }}}

    return original;
}}

// 0x7101a12020
// void App.Unit$$UpdateStateImpl (App_Unit_o *__this,bool isAutoEquip,App_UnitItem_o *equipped,MethodInfo *method)
#[skyline::hook(offset=0x01a12020)]
pub fn unit_updatestateimpl(this: &mut Unit, is_auto_equip: bool, equipped: Option<&UnitItem>, _method_info : u64)
{unsafe{
    // Unequip All Bond Ring Skills inside of equip skill array.
    for i in 0..this.fields.equip_skill.len() {
        if this.fields.equip_skill[i].get_category() == 6 {
            if let Some(equip_skill) = this.fields.equip_skill[i].get_skill() {
                this.fields.equip_skill.remove_skill(equip_skill);
            }
        }
    }

    if let Some(unit_ring) = this.get_ring() {
    if let Some(_god) = this.get_god_unit() {
        // Add Skills
        let ring_data = unit_ring.fields.data;
        let skills_to_add = ring_data.get_equip_skills();
        for x in 0..skills_to_add.len() {
            // bond_ring_skill: SkillData
            if let Some(bond_ring_skill) = skills_to_add[x as usize].get_skill() {
                let sid = bond_ring_skill.sid.to_string(); // .unwrap_or("".to_string());
                if !(sid == "SID_無し" || sid == "無し" || sid == "") {
                    // category] 6: bond ring, 11: equip
                    let category = 6;
                    this.fields.equip_skill.add_skill(bond_ring_skill, category, 0);
                }
            }
        }
    }}
    
    call_original!(this, is_auto_equip, equipped, _method_info);

    // Add Enhancements
    if let Some(unit_ring) = this.get_ring() {
    if let Some(_god) = this.get_god_unit() {
        let ring_data = unit_ring.fields.data;
        // enhancements: App_CapabilitySbyte_o
        if let Some(enhancements_unwrapped) = ringdata_getenhance(ring_data, _method_info) {
            // UnitEnhanceValues
            if let Some(calculator) = this.fields.enhance_calculator {
                // Apparently order matters... if this is done after, it will not apply?
                unitenhancecalculator_addimpl(calculator, Some(enhancements_unwrapped), false, _method_info);
                if let Some(calculator_values) = calculator.fields.values {
                    let hp_max_before_values = unit_getmaxhp(this, _method_info);
                    // This actually adds Stats
                    unitenhancevalues_add2(calculator_values, Some(enhancements_unwrapped), false,  _method_info);
                    this.fields.hp_value += cmp::max(0, unit_getmaxhp(this, _method_info) - hp_max_before_values) as u8;
                }
            }
        }
    }}// Ring & God
}}

//// HOOKS: END /////

#[skyline::main(name = "S_Support_Rings")]
pub fn main() {
    // extern "C" fn(&engage::script::EventScript)
    // EventScriptRegistrationCallback
    let fn_ptr = install_support_check_script as extern "C" fn(&engage::script::EventScript);
    cobapi::install_lua_command_registerer(fn_ptr);

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };
        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );
        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    // Install Hooks
    install_hook!(infoutil_getskilllistforunitinfo);
    install_hook!(unit_updatestateimpl);

    // checks in add skill for max count (3 now)
    // 0x7101a35fd0: 1f 05 00 71 => 1f 11, 00, 71
    Patch::in_text(0x01a35fd0).bytes(&[0x1f, 0x09, 0x00, 0x71]).unwrap();
    // 0x7101a35ff8 : 1f 05 00 71 => 1f 11, 00, 71
    Patch::in_text(0x01a35ff8).bytes(&[0x1f, 0x09, 0x00, 0x71]).unwrap();
    //// make eskill list only 5 items in the UI
    // 0x7102499c8c 40 03 40 f9 ldr super,[x26] => b  #0xdc // 7102499d68
    // Patch::in_text(0x02499c8c).bytes(&[0x37, 0x00, 0x00, 0x14]).unwrap();

    // // remove the 2nd index skip when 1st index is empty 
    // // from the equip menu
    // 0x710249d318 01 03 00 54 b.ne LAB_710249d378 => b #0x60 // LAB_710249d378
    Patch::in_text(0x0249d318).bytes(&[0x18, 0x00, 0x00, 0x14]).unwrap();
    
    // don't unset god unit when setting ring:: Skip Over it.
    // 0x7101a4e044 40 02 00 b4 cbz __this,LAB_7101a4e08c => b LAB_7101a4e08c
    Patch::in_text(0x01a4e044).bytes([0x12, 0x00, 0x00, 0x14]).unwrap();
    
    // Deserialize (TimeCrystal, map reload etc.)
    // 0x7101a51928 40 02 00 b4 cbz __this,LAB_7101a51970 => 12 00 00 14 nop 
    Patch::in_text(0x01a51928).bytes([0x12, 0x00, 0x00, 0x14]).unwrap();

    // Don't call removeold:
    // When Setting God, Don't Remove Old?
    // 0x7101d602e0 cc 01 00 94 bl App.RingSelectConfirmDialog.ConfirmYesDialogIt => 1f 20 03 d5  nop
    Patch::in_text(0x01d602e0).bytes([0x1f, 0x20, 0x03, 0xd5]).unwrap();

    // Don't Call Remove Old... when something? I forget. but I think it was important.
    // 0x7101d5fc34 77 03 00 94 bl App.RingSelectConfirmDialog.ConfirmYesDialogIt => 1f 20 03 d5 nop
    Patch::in_text(0x01d5fc34).bytes([0x1f, 0x20, 0x03, 0xd5]).unwrap();

    // Don't Clear Ring when setting God Unit (from Script?)
    // 0x71021a03e8 e8 01 00 b4 cbz x8,LAB_71021a0424 => 0f 00 00 14 b LAB_71021a0424
    Patch::in_text(0x021a03e8).bytes([0x0f, 0x00, 0x00, 0x14]).unwrap();
}

