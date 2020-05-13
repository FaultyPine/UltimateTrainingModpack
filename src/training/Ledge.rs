use smash::{hash40, phx::Hash40};
use smash::app::{self};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use crate::common::*;
use crate::common::consts::*;

pub unsafe fn force_option(
    module_accessor: &mut app::BattleObjectModuleAccessor)
{
    if StatusModule::status_kind(module_accessor) as i32 == *FIGHTER_STATUS_KIND_CLIFF_WAIT {
        if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB) {
            
            let random_frame = app::sv_math::rand(
                hash40("fighter"), 
                MotionModule::end_frame(module_accessor) as i32) as f32;

            let frame = MotionModule::frame(module_accessor) as f32;
                
            if frame == random_frame || frame > 30.0 {
                let mut status = 0;
                let ledge_case : i32;

                if (*menu).LEDGE_STATE == RANDOM_LEDGE {
                    ledge_case = app::sv_math::rand(hash40("fighter"), 4) + 2;
                } else {
                    ledge_case = (*menu).LEDGE_STATE;
                }

                match ledge_case {
                    NEUTRAL_LEDGE => status = *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
                    ROLL_LEDGE => status = *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
                    JUMP_LEDGE => status = *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
                    ATTACK_LEDGE => status = *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
                    _ => ()
                }

                StatusModule::change_status_request_from_script(module_accessor, status, true);
            }
        }
    }
}

pub unsafe fn should_perform_defensive_option(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    prev_status: i32,
    status: i32) -> bool
{
    ([*FIGHTER_STATUS_KIND_CLIFF_CLIMB,
      *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
      *FIGHTER_STATUS_KIND_CLIFF_ESCAPE]
        .iter()
        .any(|i| i == &status || i == &prev_status)
    )
    && 
    (
        WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) ||
        CancelModule::is_enable_cancel(module_accessor)
    )
}

pub unsafe fn defensive_option(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
    flag: &mut i32) 
{
    let status = StatusModule::status_kind(module_accessor) as i32;
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as i32;
    if [*FIGHTER_STATUS_KIND_CLIFF_JUMP3, 
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1].contains(&status) {
        *flag |= *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE;
    }

    if should_perform_defensive_option(module_accessor, prev_status, status) {
        perform_defensive_option(module_accessor, flag);
    }
}

pub unsafe fn check_button_on(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    button: i32) -> Option<bool>
{
    if [*CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_GUARD].contains(&button) {
        if is_training_mode() && is_operation_cpu(module_accessor) {
            let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as i32;
            let status = StatusModule::status_kind(module_accessor) as i32;
            if (*menu).DEFENSIVE_STATE == DEFENSIVE_SHIELD && 
                should_perform_defensive_option(
                    module_accessor, prev_status, status) {
                return Some(true)
            }
        }
    }

    None
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    category: i32,
    flag: &mut i32) 
{
    if (*menu).LEDGE_STATE != NONE && is_training_mode() && is_operation_cpu(module_accessor) {
        force_option(module_accessor);
        defensive_option(module_accessor, category, flag);
    }
}