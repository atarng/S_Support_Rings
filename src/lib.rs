#![feature(lazy_cell, ptr_sub_ptr)]

use cobapi::*;
use engage::{
    menu::{BasicMenu, BasicMenuItem},
    dialog::yesno::BasicDialogItemYes,
    force::*,
    gamedata::{ Gamedata, JobData, PersonData,
        unit::{Unit, GodUnit, UnitRing},
        item::*,
        ring::RingData,
        skill::*,
    },
    singleton::SingletonClass,
    script::*,
};

use skyline::{ install_hook, patching::Patch, };
use unity::prelude::*;


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

#[unity::from_offset("App", "Transporter", "GetItemCount")]
pub fn transporter_getitemcount(data: &ItemData, method_info: OptionalMethod) -> i32;

// #[unity::from_offset("App", "Unit", "HasItem")]
// 7101a417b0
#[skyline::from_offset(0x01A417B0)]
pub fn unit_hasitem(this: &Unit, item: &ItemData, method_info: OptionalMethod) -> bool;

// Maybe we should make this return the count... in the future.
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

extern "C" fn install_support_check_script(event: &EventScript) {
    event.register_function("CheckSupportRank", support_try_get);
    event.register_function("CheckPlayerOwnsItem", check_player_owns_item);
}

#[skyline::main(name = "S_Support_Rings")]
pub fn main() {
    cobapi::install_lua_command_registerer(install_support_check_script);

//================= RESEARCH_BELOW ========================================
//================= RESEARCH_BELOW ========================================
//================= RESEARCH_BELOW ========================================

    // chekcs in add skill for max count (3 now)
    // 0x7101a35fd0: 1f 05 00 71 => 1f 11, 00, 71
    Patch::in_text(0x01a35fd0).bytes(&[0x1f, 0x09, 0x00, 0x71]).unwrap();
    // 0x7101a35ff8 : 1f 05 00 71 => 1f 11, 00, 71
    Patch::in_text(0x01a35ff8).bytes(&[0x1f, 0x09, 0x00, 0x71]).unwrap();

    //// make eskill list only 5 items in the UI
    // 0x7102499c8c 40 03 40 f9 ldr super,[x26]=>App.SkillEditEquipSkillMenuItem_T
    //                          b  #0xdc // b 7102499d68
    Patch::in_text(0x02499c8c).bytes(&[0x37, 0x00, 0x00, 0x14]).unwrap();

    // Don't clear owner from Ring
    // 0x7101d60b18 62 f3 fb 17 b App.UnitRingPool$$ClearOwner
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // 0x7101d60b18 c0 03 5f d6
    // Patch::in_text(0x01d60b18).bytes([0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    
    // don't unset god unit when setting ring.
    // 0x7101a4e044 40 02 00 b4 cbz __this,LAB_7101a4e08c
    // 0x7101a4e04c 1d c8 23 94 bl App.GodUnit$$ClearParent
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // 0x7101a4e044:  12 00 00 14 b LAB_7101a4e08c
    Patch::in_text(0x01a4e044).bytes([0x12, 0x00, 0x00, 0x14]).unwrap();

    // Don't call removeold:
    // When trying to remove self ring: call remove old!!!
    // 0x7101d604bc 55 01 00 94     bl         App.RingSelectConfirmDialog.ConfirmYesDialogIt
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>
    // 0x7101d604bc 1f 20 03 d5     nop
    // Patch::in_text(0x01d604bc).bytes([0x1f, 0x20, 0x03, 0xd5]).unwrap();

    // Don't call removeold:
    // When Setting God, Don't Remove Old?
    // 0x7101d602e0 cc 01 00 94 bl App.RingSelectConfirmDialog.ConfirmYesDialogIt void App.RingSelectConfirmDialog
    // >>>>>>>>>>>>>>>>>>>>>>>>>>
    // 0x7101d602e0 cc 01 00 94  nop
    Patch::in_text(0x01d602e0).bytes([0x1f, 0x20, 0x03, 0xd5]).unwrap();

    // Don't Clear Ring when setting God Unit (from Script?)
    // 71021a03e8 e8 01 00 b4 cbz x8,LAB_71021a0424
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // 71021a03e8 0f 00 00 14 b LAB_71021a0424
    Patch::in_text(0x021a03e8).bytes([0x0f, 0x00, 0x00, 0x14]).unwrap();

    // Try skipping UnitActor reload when setting ring, just to see what happens.
    // 0x7101a4e0b0 a4 66 14 14 b App.UnitActor$$Reload
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // 0x7101a4e0b0 1f 20 03 d5 nop
    // Patch::in_text(0x01a4e0b0).bytes([0x1f, 0x20, 0x03, 0xd5]).unwrap();

    install_hook!(godunit_clearchild);
    install_hook!(godunit_clearparent);

    install_hook!(infoutil_getskilllistforunitinfo);

    install_hook!(ringdata_getequipskills);
    install_hook!(ringdata_getequipsids);

    install_hook!(rscd_confirmyesdialogitem_removeold);
    install_hook!(rscd_confirmyesdialogitem_acall);
    
    install_hook!(unit_addequipskill2);
    install_hook!(unit_removeequipskill2);
    install_hook!(unit_getequipskill);
    install_hook!(unit_addskill1);
    install_hook!(unit_addskill2);
    install_hook!(unit_addskill3);
    install_hook!(unit_addskillwoupdate);
    install_hook!(unit_cleargodunit);
    install_hook!(unit_cleargodunitfromcopy);
    install_hook!(unit_clearring);
    install_hook!(unit_tryconnectgodunit);
    install_hook!(unit_tryconnectgodunittocopy);
    install_hook!(unit_trydisconnectgodunit);
    install_hook!(unit_trydisconnectring);
    install_hook!(unit_setgodunit_1);
    install_hook!(unit_setring);

    // Does this get Called?
    install_hook!(unitring_changeowner);

    install_hook!(unitringpool_clearowner);
    install_hook!(unitringpool_setowner);

    install_hook!(unitinfoparammanager_setunit);

    install_hook!(unitselectringmenu_takeoffallrings);
    install_hook!(unitselectringmenu_takeoffring);

    install_hook!(unitstatussetter_setskill);

}

//================= RESEARCH_BELOW ========================================
//================= RESEARCH_BELOW ========================================
//================= RESEARCH_BELOW ========================================

// void App.ScriptUnit$$UnitSetGodUnit(MoonSharp_Interpreter_DynValue_array *args,MethodInfo *method)
// 0x71021a02a0
// NO CLUE IF THIS IS CORRECT.
// #[skyline::hook(offset=0x021a02a0)]
// pub fn scriptunit_unitsetgodunit(args: &Il2CppArray<DynValue>, _method_info : u64)
// {
//     println!("[scriptunit_unitsetgodunit]");
//     call_original!(args, _method_info);
// }

#[unity::class("App", "UnitSelectRingMenu")]
pub struct UnitSelectRingMenu {
    base: BasicMenu<BasicMenuItem>, // I call it base but you can call it super if you want, it doesn't matter
    close_event_handler: *const u8  // can also represent as u64, what matters is the type takes the same amount of space
}

#[unity::hook("App", "UnitSelectRingMenu", "TakeOffAllRings")]
pub fn unitselectringmenu_takeoffallrings(this: &UnitSelectRingMenu, method_info: OptionalMethod) {
    println!("[unitselectringmenu_takeoffallrings]");
    call_original!(this, method_info);
}

#[unity::hook("App", "UnitSelectRingMenu", "TakeOffRing")]
pub fn unitselectringmenu_takeoffring(this: &UnitSelectRingMenu, unit: Option<&Unit>, method_info: OptionalMethod) {
    // let _pid = unit.get_pid().inspect(|pid| println!("[unitselectringmenu_takeoffring] {pid}"))
    //     .expect("[unitselectringmenu_takeoffring] unit does not have pid.");
    call_original!(this, unit, method_info);
}


#[unity::class("App", "RingSelectConfirmDialog")]
pub struct RingSelectConfirmDialog {}

#[unity::class("RingSelectConfirmDialog", "ConfirmYesDialogItem")]
pub struct ConfirmYesDialogItem {
    base: BasicDialogItemYes
}

// God Unit has both a parent and child??
// 0x7102340150
// void App.GodUnit$$ClearChild(App_GodUnit_o *__this,MethodInfo *method)
#[skyline::hook(offset=0x02340150)]
pub fn godunit_clearchild(this: &GodUnit, method_info: OptionalMethod) {
    println!("[godunit_clearchild]");
    call_original!(this, method_info);
}

// 0x71023400c0
#[skyline::hook(offset=0x023400c0)]
pub fn godunit_clearparent(this: &GodUnit, method_info: OptionalMethod) {
    println!("[godunit_clearparent] gid: {}", this.fields.data.fields.gid);
    call_original!(this, method_info);
}

// System_String_array * App.RingData$$get_EquipSids(App_RingData_o *__this,MethodInfo *method)
// 0x71024246d0
// NO CLUE IF THIS IS CORRECT
#[skyline::hook(offset=0x024246d0)]
pub fn ringdata_getequipsids(this: Option<&RingData>, method_info: OptionalMethod) -> &'static mut Array<&Il2CppString> {
    println!("[ringdata_getequipsids]");
    return call_original!(this, method_info);
}

// App_InfoUtil_StatusSkill_array * ::::: &'static mut Array<&'static StatusSkill>
// App_SkillArray_o * App.RingData$$get_EquipSkills(App_RingData_o *__this,MethodInfo *method)
// 0x71024246f0
// NO CLUE IF THIS IS CORRECT
#[skyline::hook(offset=0x024246f0)]
pub fn ringdata_getequipskills(this: Option<&RingData>, method_info: OptionalMethod) -> Option<&'static SkillArray> {
    println!("[ringdata_getequipskills]");
    return call_original!(this, method_info);
}

// 0x710233d8f0
// bool App.GodUnitSelectMenu$$CreateBind
//                (App_ProcInst_o *super,UnityEngine_GameObject_o *listRootObject,
//                App_GodUnitSelectMenu_SelectEventHandler_o *selectEventHandler,
//                App_GodUnitSelectMenu_DecideEventHandler_o *decideEventHandler,
//                App_GodUnit_o *selectedGod,MethodInfo *method)


// int32_t App.RingSelectConfirmDialog.ConfirmYesDialogItem$$ACall
//                   (App_RingSelectConfirmDialog_ConfirmYesDialogItem_o *__this,MethodInfo *method)
// 0x7101d5fa10
#[skyline::hook(offset=0x01d5fa10)]
pub fn rscd_confirmyesdialogitem_acall(this: &ConfirmYesDialogItem, method_info: OptionalMethod) -> i32 {
    println!("[rscd_confirmyesdialogitem_acall]");
    return call_original!(this, method_info);
}

// This method is currently pretty buggy.
// #[unity::hook("App", "RingSelectConfirmDialog.ConfirmYesDialogItem")]
// Can't use unity::hook, because of nested struct/class
// 0x7101d60a10
#[skyline::hook(offset=0x01D60A10)]
pub fn rscd_confirmyesdialogitem_removeold(this: &ConfirmYesDialogItem, unit: Option<&Unit>, method_info: OptionalMethod) {
    // let cached_ring = unit.get_ring();
    // match cached_ring {
    //     Some(_value) => { println!("[confirmyesdialogitem_removeold] There is a ring available.") }
    //     None => { println!("[confirmyesdialogitem_removeold] No ring was assigned.") }
    // }
    // let cached_god = unit.get_god_unit();
    // match cached_god {
    //     Some(god_unwrapped) => {
    //         println!("[confirmyesdialogitem_removeold] God Exists: {0}",
    //                 god_unwrapped.fields.data.get_ascii_name().unwrap())
    //     }
    //     None => { println!("[confirmyesdialogitem_removeold] No God was assigned.") }
    // }

    match unit {
        Some(unit_unwrapped) => {
            // for some reason getting pid sems to crash.
            // let person = &unit_unwrapped.fields.person;
            // println!("[confirmyesdialogitem_removeold] person addres: {person:p}");
            let cached_ring = unit_unwrapped.get_ring();
            match cached_ring {
                Some(_ring_unwrapped) => {
                    // This does cause a panic attack when trying to take an emblem ring (from someone else?)
                    // println!("[confirmyesdialogitem_removeold] Ring Exists: {}", ring_unwrapped.data.name);
                    println!("[rscd_confirmyesdialogitem_removeold] Ring Exists");
                }
                None => { println!("[rscd_confirmyesdialogitem_removeold] No ring was assigned.") }
            }
            let cached_god = unit_unwrapped.get_god_unit();
            match cached_god {
                Some(_god_unwrapped) => { println!("[rscd_confirmyesdialogitem_removeold] God Exists.") }
                None => { println!("[rscd_confirmyesdialogitem_removeold] No God was assigned.") }
            }

            // let _crash_pid = unit_unwrapped.get_pid().
            //         inspect(|crash_pid| println!("[confirmyesdialogitem_removeold] pid: {crash_pid}"));
            // println!("[confirmyesdialogitem_removeold] level: {}", unit_unwrapped.level)
        } 
        None => { println!("[rscd_confirmyesdialogitem_removeold] no unit." ) }
    }

    call_original!(this, unit, method_info);
}


#[unity::class("App", "UnitRingPool")]
pub struct UnitRingPool {
     base: SingletonClass
}

// #[unity::hook("App", "UnitRingPool", "ClearOwner")]
// this: &UnitRingPool,
// static method
// clear owner: 0x7101c5d8a0
#[skyline::hook(offset=0x01C5D8A0)]
pub fn unitringpool_clearowner(ring: Option<&UnitRing>, method_info: OptionalMethod) {
    // println!("[unitringpool_clearowner] skip");
//===============================================
    match ring {
        Some(ring_unwrapped) => {
            println!("[unitringpool_clearowner] has ring: {0}", ring_unwrapped.data.name);
            // ring_unwrapped.owner : Unit
            match ring_unwrapped.owner {
                Some(owner_unwrapped) => {
                    // let _pid = owner_unwrapped.get_pid().
                            // inspect(|pid| println!("[unitringpool_clearowner] Owner being cleared: {pid}"));
                    ////////////////////////////////////////
                    let pid = owner_unwrapped.get_pid();
                    println!("[unitringpool_clearowner] Owner being cleared: {}", pid);
                }
                None => {
                    println!("[unitringpool_clearowner] No Owner");
                }
            }
        }
        None => { println!("[unitringpool_clearowner] no ring"); }
    }
    call_original!(ring, method_info);
}

// #[unity::hook("App", "UnitRingPool", "SetOwner")]
// this: &UnitRingPool,
// static method
// clear owner: 7101c5d760
#[skyline::hook(offset=0x01C5D760)]
pub fn unitringpool_setowner(ring: &UnitRing, owner: Option<&Unit>, method_info: OptionalMethod) {
    println!("[unitringpool_setowner] ring: {0}", ring.data.name);

    let ring_previous_owner = ring.owner.is_some();
    let incoming_owner = owner.is_some();

    if ring_previous_owner && incoming_owner {
        println!("[unitringpool_setowner] Previous Owner: {0} Incoming Owner: {1}",
                ring.owner.unwrap().get_pid(),
                owner.unwrap().get_pid());
    } else if incoming_owner {
        println!("[unitringpool_setowner] NoPreviousOwner IncomingOwner: {0}",
                owner.unwrap().get_pid());
    } else if ring_previous_owner {
        println!("[unitringpool_setowner] PreviousOwner: {0} NoIncomingOwner",
                ring.owner.unwrap().get_pid());
    } else {
        println!("[unitringpool_setowner] NoPreviousOwner and NoIncomingOwner.");
    }

    call_original!(ring, owner, method_info);
}

// 0x7101FC7300
// bool App.InfoUtil.StatusSkill$$get_IsActive(App_InfoUtil_StatusSkill_o *__this,MethodInfo *method)
#[skyline::from_offset(0x1FC7300)]
pub fn infoutil_statusskill_getisactive(this: &StatusSkill, _method_info: u64) -> bool;

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_IsActive")]
#[skyline::from_offset(0x1FC7310)]
pub fn infoutil_statusskill_setisactive(this: &StatusSkill, active: bool, _method_info: u64);

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_Data")]
#[skyline::from_offset(0x1FC72F0)]
pub fn infoutiil_statusskill_setdata(this: &StatusSkill, value: Option<&SkillData>, _method_info: u64);

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_Category")]
#[skyline::from_offset(0x1FC7330)]
pub fn infoutil_statusskill_setcategory(this: &StatusSkill, cat: i32, _method_info: u64);

// int32_t App.InfoUtil.StatusSkill$$get_Category (App_InfoUtil_StatusSkill_o *__this,MethodInfo *method)
//0x7101FC7320
#[skyline::from_offset(0x1FC7320)]
pub fn infoutil_statusskill_getcategory(this: &StatusSkill, _method_info: u64) -> i32;

// App_InfoUtil_StatusSkill_array * ::::: &'static mut Array<&'static StatusSkill>
// 0x7102911030
#[unity::hook("App", "InfoUtil", "GetSkillListForUnitInfo")]
pub fn infoutil_getskilllistforunitinfo(unit: Option<&Unit>, is_equip: bool, is_pack: bool, mut size: i32, _method_info : u64) ->
        &'static mut Array<&'static StatusSkill>
{unsafe{
    println!("[infoutil_getskilllistforunitinfo] is_equip: {} is_pack: {} size: {}", is_equip, is_pack, size);

    //// change original method behavior:
    // size: 10 -> 11
    size = 11;
    let original: &'static mut Array<&'static StatusSkill> = call_original!(unit, is_equip, is_pack, size, _method_info);

    if let Some(person) = unit {
        if let Some(god) = person.get_god_unit() {
            if let Some(unit_ring) = person.get_ring() {
                let ring_data = unit_ring.fields.data;
                println!("[infoutil_getskilllistforunitinfo] god: {} ring: {}", god.fields.data.fields.gid, ring_data.fields.rid);

                if let Some(equipped_skills) = unit_getequipskill(person, _method_info) {

                    println!("[infoutil_getskilllistforunitinfo] unit_equipped_skills: {}", equipped_skills.len());

                    let mut start = 0;
                    let mut fin = size as usize;
                    let mut offset = 0;

                    if is_equip {
                        //realstart = 0
                        start = 2;
                        fin = 3;

                    // } else { // if is_pack
                    //     // if job skill present: set offset 2 to skill personal and job.
                    //     // otherwise offset 1 to skip personal only (?)
                    //     if infoutil_statusskill_getcategory(original[1], _method_info) == 2 && infoutil_statusskill_getisactive(original[1], _method_info) {
                    //         offset = 2;
                    //         start = 2;
                    //         fin = 3; // 7 => 5
                    //     } else {
                    //         offset = 1;
                    //         start = 1;
                    //         fin = 2; // 6 => 4
                    //     }
                    // }
                    // // make room for the new bond ring skill slot.
                    // // start from end (11) -> fin(4/5) {11, 10, 9 ... 4/5}
                    // for x in (fin..original.len()).rev() {
                    //     original[x] = original[x-1];
                    // }
                    // let skills_to_add = ring_data.get_equip_skills();
                    // println!("[infoutil_getskilllistforunitinfo] ringskills_count: {}", skills_to_add.len());
                    // // 0..skills_to_add.len()
                    // for x in start..fin {
                    //     // bond_ring_skill: SkillData
                    //     if let Some(bond_ring_skill) = skills_to_add[x - offset as usize].get_skill() {
                    //         let dupet = Il2CppClass::from_name("App", "InfoUtil").unwrap().get_nested_types().iter().find(|x| x.get_name() == "StatusSkill").unwrap();
                    //         let newt: &'static StatusSkill = il2cpp::instantiate_class::<StatusSkill>(dupet).unwrap();
                    //         original[x as usize] = newt;
                    //         // Category:
                    //         // 6: ring
                    //         // 11: Inheritance
                    //         infoutil_statusskill_setcategory(original[x as usize], 6, _method_info); 
                    //         let sid = bond_ring_skill.sid.get_string().unwrap_or("".to_string());
                    //         if sid == "SID_無し" || sid == "無し" || sid == "" {
                    //             infoutiil_statusskill_setdata(original[x as usize], None, _method_info);
                    //             infoutil_statusskill_setisactive(original[x as usize], false, _method_info);
                    //         } else {
                    //             println!("[infoutil_getskilllistforunitinfo] {0}: {1}", x, sid);
                    //             infoutiil_statusskill_setdata(original[x as usize], Some(bond_ring_skill), _method_info);
                    //             infoutil_statusskill_setisactive(original[x as usize], true, _method_info);
                    //         }
                    //     }
                    // }
                    ///////////////////////////////////////////////////////////////////////////////////////

                    } else { // if is_pack
                        // if job skill present: set offset 2 to skill personal and job.
                        // otherwise offset 1 to skip personal only (?)
                        if infoutil_statusskill_getcategory(original[1], _method_info) == 2 && infoutil_statusskill_getisactive(original[1], _method_info) {
                            offset = 2;
                            start = 2;
                            fin = 5; // 7 => 5
                        } else {
                            offset = 1;
                            start = 1;
                            fin = 4; // 6 => 4
                        }
                    }
                    for x in (fin..original.len()).rev() {
                        original[x] = original[x-1];
                    }
                    for x in start..fin {
                        if let Some(eSkill) = equipped_skills[x - offset as usize].get_skill() {
                            let dupet = Il2CppClass::from_name("App", "InfoUtil").unwrap().get_nested_types().iter().find(|x| x.get_name() == "StatusSkill").unwrap();
                            let newt: &'static StatusSkill = il2cpp::instantiate_class::<StatusSkill>(dupet).unwrap();
                            original[x as usize] = newt;
                            // category 11: equip skills
                            // let skill_category = 11;
                            let skill_category = equipped_skills[x as usize].get_category();
                            infoutil_statusskill_setcategory(original[x as usize], skill_category, _method_info); 
                            let sid = eSkill.sid.get_string().unwrap_or("".to_string());
                            if sid == "SID_無し" || sid == "無し" || sid == "" {
                                infoutiil_statusskill_setdata(original[x as usize], None, _method_info);
                                infoutil_statusskill_setisactive(original[x as usize], false, _method_info);
                            } else {
                                println!("[infoutil_getskilllistforunitinfo] {0}: {1} category: {2}", x, sid, skill_category);
                                infoutiil_statusskill_setdata(original[x as usize], Some(eSkill), _method_info);
                                infoutil_statusskill_setisactive(original[x as usize], true, _method_info);
                            }
                        }
                    }
                }
            }
        }
    }

    return original;
}}


// bool App.Unit$$AddSkillWithoutUpdate (App_Unit_o *__this,App_SkillData_o *skill,int32_t category,int32_t age, MethodInfo *method)
// 0x7101a20ec0
#[skyline::hook(offset=0x01a20ec0)]
pub fn unit_addskillwoupdate(this: &Unit, skill: Option<&SkillData>, category: i32, age: i32, _method_info : u64)
{
    println!("[unit_addskillwoupdate] unit: {} category: {}", this.get_pid(), category);
    call_original!(this, skill, category, age, _method_info);
}

// void App.Unit$$AddSkill(App_Unit_o *__this,App_SkillArray_o *array,MethodInfo *method)
// 0x7101a1f620
#[skyline::hook(offset=0x01a1f620)]
pub fn unit_addskill1(this: &Unit, array: Option<&SkillArray>, _method_info : u64)
{
    println!("[unit_addskill1] {}", this.get_pid());
    call_original!(this, array, _method_info);
}

// void App.Unit$$AddSkill(App_Unit_o *__this,App_SkillData_o *skill,int32_t category, MethodInfo *method)
// 0x7101a1ff40
#[skyline::hook(offset=0x01a1ff40)]
pub fn unit_addskill2(this: &Unit, skill: Option<&SkillData>, category: i32, _method_info : u64)
{
    println!("[unit_addskill2] unit: {0} category: {1}", this.get_pid(), category);
    call_original!(this, skill, category, _method_info);
}

// void App.Unit$$AddSkill(App_Unit_o *__this,App_SkillData_o *skill,int32_t category,int32_t age, MethodInfo *method)
// 0x7101a20820
#[skyline::hook(offset=0x01a20820)]
pub fn unit_addskill3(this: &Unit, skill: Option<&SkillData>, category: i32, age: i32, _method_info : u64)
{
    println!("[unit_addskill3] unit: {0} category: {1} age: {2}", this.get_pid(), category, age);
    call_original!(this, skill, category, age, _method_info);
}

#[unity::hook("App", "Unit", "ClearGodUnit")]
pub fn unit_cleargodunit(this: &Unit, _method_info : u64)
{
    println!("[unit_cleargodunit] {}", this.get_pid());
    call_original!(this, _method_info);
}

#[unity::hook("App", "Unit", "ClearGodUnitFromCopy")]
pub fn unit_cleargodunitfromcopy(this: &Unit, _method_info : u64)
{
    println!("[unit_cleargodunitfromcopy] {0} ", this.get_pid());
    call_original!(this, _method_info);
}

// Used Pretty much everywhere:
// Involved in clearing unit for previews when selecting emblem/bond ring.
#[unity::hook("App", "Unit", "ClearRing")]
pub fn unit_clearring(this: &Unit, _method_info : u64)
{
    // println!("[unit_clearring] Unit: {0} SKIPCLEARRING", this.get_pid());
    //===================================================================
    println!("[unit_clearring] Unit: {0} ClearRing", this.get_pid());
    call_original!(this, _method_info);
}

// Only seeing this called on launch.
#[unity::hook("App", "UnitRing", "ChangeOwner")]
pub fn unitring_changeowner(this: &UnitRing, owner: &Unit, _method_info: u64)
{
    println!("[unitring_changeowner] Ring: {0}", this.data.name);
    match this.owner {
        Some(owner_unwrapped) => {
            println!("[unitring_changeowner] Previous Owner: {0}", owner_unwrapped.get_pid())
        }
        None => {
            println!("[unitring_changeowner] No Owner");
        }
    }
    println!("[unitring_changeowner] Change owner to: {0}", owner.get_pid());
    call_original!(this, owner, _method_info);
}

// unity::from_offset("App", "Unit", "get_EquipSkill")
// 0x7101a54ee0
#[skyline::hook(offset=0x01a54ee0)]
pub fn unit_getequipskill(this: &Unit, _method_info: u64) -> Option<&SkillArray> {
    println!("[unit_getequipskill]");

    let equipped_skills = call_original!(this, _method_info);

    if let Some(skills_unwrapped) = equipped_skills {
        println!("[unit_getequipskill] skills: {0}", skills_unwrapped.len());
        // let mut skill_count = 0;
        // for x in 0..skills_unwrapped.len() {
        //     if let Some(bond_ring_skill) = skills_unwrapped[x as usize].get_skill() {
        //         let sid = bond_ring_skill.sid.get_string().unwrap_or("".to_string());
        //         if sid == "SID_無し" || sid == "無し" || sid == "" {
        //         } else {
        //             skill_count += 1;
        //         }
        //     }
        // }
        let mut ring_skill_added = 0;
        // if skill_count < 3 {
        //     println!("[unit_getequipskill] skill count is {}, we've probably already added bond ring skill.", skill_count);
        // } else {
        if let Some(god) = this.get_god_unit() {
        if let Some(unit_ring) = this.get_ring() {
            let ring_data = unit_ring.fields.data;
            let skills_to_add = ring_data.get_equip_skills();
            println!("[unit_getequipskill] ring: {} skills: {}", ring_data.fields.rid, skills_to_add.len());
            for x in 0..skills_to_add.len() {
                // bond_ring_skill: SkillData
                if let Some(bond_ring_skill) = skills_to_add[x as usize].get_skill() {
                    let sid = bond_ring_skill.sid.get_string().unwrap_or("".to_string());
                    if sid == "SID_無し" || sid == "無し" || sid == "" {
                    } else {
                        println!("[unit_getequipskill] add_skill_{}: {}", x, sid);
                        // category:
                        // 6: bond ring
                        // 11: equip
                        let category = 6;
                        if ring_skill_added == 0 {
                            skills_unwrapped.add_skill(bond_ring_skill, category, 0);
                        }
                        // unit_addequipskill2(this, skills_to_add[x as usize].get_skill(), _method_info);
                        ring_skill_added += 1;
                    }
                }
            }
        }}
        // }
    } else {
        println!("[unit_getequipskill] ????????????????");
    }

    return equipped_skills;
}

// 0x7101a4f180
#[unity::hook("App", "Unit", "SetGodUnit")]
pub fn unit_setgodunit_1(this: &Unit, god_unit: &GodUnit, _method_info : u64)
{
    println!("[unit_setgodunit_1] {0}", this.get_pid());
    call_original!(this, god_unit, _method_info);
}

#[unity::hook("App", "Unit", "SetRing")]
pub fn unit_setring(this: &Unit, ring: &UnitRing, _method_info : u64)
{
    let units_ring = this.get_ring();
    match units_ring {
        Some(unwrapped_ring) => {
            println!("[unit_setring] {0} has an existing ring: {1}", this.get_pid(), unwrapped_ring.data.name);
        } None => {
            println!("[unit_setring] {0} does not have an existing ring.", this.get_pid());
        }
    }
    match ring.owner {
        Some(owner_unwrapped) => {
            println!("[unit_setring] Ring {0} Owner: {1}", ring.data.name, owner_unwrapped.get_pid());
        }
        None => {
            println!("[unit_setring] Ring {0} NoOwner", ring.data.name);
        }
    }

    call_original!(this, ring, _method_info);

    // if let Some(god_unwrapped) = this.get_god_unit() {
    //     let skills_to_add = ring.fields.data.get_equip_skills();
    //     println!("[unit_setring] ringskills_count: {}", skills_to_add.len());
    //     for x in 0..skills_to_add.len() {
    //         // bond_ring_skill: SkillData
    //         if let Some(bond_ring_skill) = skills_to_add[x as usize].get_skill() {
    //             let sid = bond_ring_skill.sid.get_string().unwrap_or("".to_string());
    //             if sid == "SID_無し" || sid == "無し" || sid == "" {
    //             } else {
    //                 println!("[unit_setring] {0}: {1}", x, sid);
    //                 unit_addequipskill2(this, skills_to_add[x as usize].get_skill(), _method_info);
    //                 break;
    //             }
    //         }
    //     }
    // }
}

#[unity::hook("App", "Unit", "TryConnectGodUnit")]
pub fn unit_tryconnectgodunit(this: &Unit, god_unit: &GodUnit, _method_info : u64) -> Option<&'static GodUnit>
{
    println!("[unit_tryconnectgodunit] {0}", this.get_pid());
    return call_original!(this, god_unit, _method_info);
}

#[unity::hook("App", "Unit", "TryConnectGodUnitToCopy")]
pub fn unit_tryconnectgodunittocopy(this: &Unit, god_unit: &GodUnit, _method_info : u64) -> Option<&'static GodUnit>
{
    println!("[unit_tryconnectgodunittocopy] {0}", this.get_pid());
    return call_original!(this, god_unit, _method_info);
}

#[unity::hook("App", "Unit", "TryDisconnectGodUnit")]
pub fn unit_trydisconnectgodunit(this: &Unit, _method_info : u64) -> Option<&'static GodUnit>
{
    println!("[unit_trydisconnectgodunit] {0}", this.get_pid());
    return call_original!(this, _method_info);
}

#[unity::hook("App", "Unit", "TryDisconnectRing")]
pub fn unit_trydisconnectring(this: &Unit, _method_info : u64) -> Option<&'static UnitRing>
{
    // if let Some(god_unwrapped) = this.get_god_unit() {
    // if let Some(ring_unwrapped) = this.get_ring() {
    //     let skills_to_add = ring_unwrapped.fields.data.get_equip_skills();
    //     println!("[unit_setring] ringskills_count: {}", skills_to_add.len());
    //     for x in 0..skills_to_add.len() {
    //         // bond_ring_skill: SkillData
    //         if let Some(bond_ring_skill) = skills_to_add[x as usize].get_skill() {
    //             let sid = bond_ring_skill.sid.get_string().unwrap_or("".to_string());
    //             if sid == "SID_無し" || sid == "無し" || sid == "" {
    //             } else {
    //                 println!("[unit_setring] {0}: {1}", x, sid);
    //                 unit_removeequipskill2(this, skills_to_add[x as usize].get_skill(), _method_info);
    //                 break;
    //             }
    //         }
    //     }
    // }}

    println!("[unit_trydisconnectring] {0}", this.get_pid());
    return call_original!(this, _method_info);
}

//bool App.Unit$$AddEquipSkill(App_Unit_o *__this,App_SkillData_o *skill,MethodInfo *method)
// 0x7101a35f80
#[skyline::hook(offset=0x01a35f80)]
pub fn unit_addequipskill2(this: &Unit, skill: Option<&SkillData>, _method_info: u64) -> bool {
    if let Some(skill_unwrapped) = skill {
        println!("[unit_addequipskill2] pid: {0} skill: {1}", this.get_pid(), skill_unwrapped.sid);
    } else {
        println!("[unit_addequipskill2] {}", this.get_pid());
    }
    return call_original!(this, skill, _method_info);
}

// void App.Unit$$RemoveEquipSkill(App_Unit_o *__this,App_SkillData_o *skill,MethodInfo *method)
// 0x7101a36f10
#[skyline::hook(offset=0x01a36f10)]
pub fn unit_removeequipskill2(this: &Unit, skill: Option<&SkillData>, _method_info: u64) {
    if let Some(skill_unwrapped) = skill {
        println!("[unit_removeequipskill2] pid: {0} skill: {1}", this.get_pid(), skill_unwrapped.sid);
    } else {
        println!("[unit_removeequipskill2] {}", this.get_pid());
    }
    call_original!(this, skill, _method_info);
}

#[unity::class("App", "UnitInfoParamManager")]
pub struct UnitInfoParamManager {
}

// void App.UnitInfoParamManager$$SetUnit
// (App_UnitInfoParamManager_o *__this,App_Unit_o *unit,int32_t x,int32_t z,
// bool isDiffCollect,int32_t f,bool isGodChange,App_GodUnit_o *god,App_UnitRing_o *ring
// ,MethodInfo *method)
// 0x7101f8cd80
#[skyline::hook(offset=0x01f8cd80)]
pub fn unitinfoparammanager_setunit(this: &UnitInfoParamManager, unit: Option<&Unit>, x: i32, z: i32, is_diff_collect: bool,
        f: i32, is_god_change: bool, god: Option<&GodUnit>, ring: Option<&UnitRing>, _method_info: u64) {
    println!("[unitinfoparammanager_setunit] ({0}, {1}) is_diff: {2} f: {3} is_god_change: {4}",
            x, z, is_diff_collect, f, is_god_change);
    if let Some(unit_unwrapped) = unit {
        println!("[unitinfoparammanager_setunit] {}", unit_unwrapped.get_pid());
        if let Some(god_unwrapped) = god {
            println!("[unitinfoparammanager_setunit] god_gid: {}", god_unwrapped.fields.data.fields.gid);
        }
        if let Some(ring_unwrapped) = ring {
            println!("[unitinfoparammanager_setunit] ring_rid: {}", ring_unwrapped.fields.data.fields.rid);
        }
    } else {
        println!("[unitinfoparammanager_setunit] no unit?");
    }

    call_original!(this, unit, x, z, is_diff_collect, f, is_god_change, god, ring, _method_info);
}

#[unity::class("App", "UnitStatusSetter")]
pub struct UnitStatusSetter {
}

// void App.UnitStatusSetter$$SetSkill
//                (App_UnitStatusSetter_o *__this,App_Unit_o *unit,MethodInfo *method)
// 0x7101c69670
#[skyline::hook(offset=0x01c69670)]
pub fn unitstatussetter_setskill(this: &UnitStatusSetter, unit: Option<&Unit>, _method_info: u64) {
    if let Some(unit_unwrapped) = unit {
        println!("[unitstatussetter_setskill] unit: {}", unit_unwrapped.get_pid());
    } else {
        println!("[unitstatussetter_setskill] no unit");
    }

    call_original!(this, unit, _method_info);
}
