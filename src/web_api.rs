use crate::animation::AnimationMode;
use crate::types::{ColorScheme, RenderMode};

pub enum WebCommand {
    LoadPdbText(String),
    SetStructureLabel(String),
    LoadDefaultStructure,
    SetColorScheme(ColorScheme),
    SetRenderMode(RenderMode),
    NextTheme,
    PreviousTheme,
    SetThemeIndex(usize),
    SetRadiusScale(f32),
    ResetView,
    SetAnimationEnabled(bool),
    SetAnimationMode(AnimationMode),
    SetAnimationSpeed(f32),
    SetUiVisible(bool),
}

#[cfg(target_arch = "wasm32")]
mod imp {
    use super::WebCommand;
    use crate::animation::AnimationMode;
    use crate::types::{ColorScheme, RenderMode};

    const CMD_LOAD_PDB_TEXT: u32 = 1;
    const CMD_SET_COLOR_SCHEME: u32 = 2;
    const CMD_SET_RENDER_MODE: u32 = 3;
    const CMD_NEXT_THEME: u32 = 4;
    const CMD_PREV_THEME: u32 = 5;
    const CMD_SET_RADIUS_SCALE: u32 = 6;
    const CMD_RESET_VIEW: u32 = 7;
    const CMD_SET_ANIMATION_ENABLED: u32 = 8;
    const CMD_SET_ANIMATION_MODE: u32 = 9;
    const CMD_SET_ANIMATION_SPEED: u32 = 10;
    const CMD_SET_UI_VISIBLE: u32 = 11;
    const CMD_SET_STRUCTURE_LABEL: u32 = 12;
    const CMD_LOAD_DEFAULT_STRUCTURE: u32 = 13;
    const CMD_SET_THEME_INDEX: u32 = 14;

    unsafe extern "C" {
        fn protdot_command_pending() -> u32;
        fn protdot_command_kind() -> u32;
        fn protdot_command_string_len() -> usize;
        fn protdot_command_string_copy(ptr: *mut u8, max_len: usize);
        fn protdot_command_f32() -> f32;
        fn protdot_command_u32() -> u32;
        fn protdot_command_consume();
    }

    fn take_string() -> String {
        let len = unsafe { protdot_command_string_len() };
        let mut buf = vec![0u8; len];

        if len > 0 {
            unsafe {
                protdot_command_string_copy(buf.as_mut_ptr(), len);
            }
        }

        String::from_utf8(buf)
            .unwrap_or_else(|err| String::from_utf8_lossy(&err.into_bytes()).to_string())
    }

    pub fn poll_commands() -> Vec<WebCommand> {
        let mut commands = Vec::new();

        loop {
            if unsafe { protdot_command_pending() } == 0 {
                break;
            }

            let kind = unsafe { protdot_command_kind() };
            let command = match kind {
                CMD_LOAD_PDB_TEXT => Some(WebCommand::LoadPdbText(take_string())),
                CMD_SET_COLOR_SCHEME => ColorScheme::from_index(unsafe { protdot_command_u32() })
                    .map(WebCommand::SetColorScheme),
                CMD_SET_RENDER_MODE => RenderMode::from_index(unsafe { protdot_command_u32() })
                    .map(WebCommand::SetRenderMode),
                CMD_NEXT_THEME => Some(WebCommand::NextTheme),
                CMD_PREV_THEME => Some(WebCommand::PreviousTheme),
                CMD_SET_RADIUS_SCALE => {
                    Some(WebCommand::SetRadiusScale(unsafe { protdot_command_f32() }))
                }
                CMD_RESET_VIEW => Some(WebCommand::ResetView),
                CMD_SET_ANIMATION_ENABLED => Some(WebCommand::SetAnimationEnabled(
                    unsafe { protdot_command_u32() } != 0,
                )),
                CMD_SET_ANIMATION_MODE => {
                    AnimationMode::from_index(unsafe { protdot_command_u32() })
                        .map(WebCommand::SetAnimationMode)
                }
                CMD_SET_ANIMATION_SPEED => Some(WebCommand::SetAnimationSpeed(unsafe {
                    protdot_command_f32()
                })),
                CMD_SET_UI_VISIBLE => Some(WebCommand::SetUiVisible(
                    unsafe { protdot_command_u32() } != 0,
                )),
                CMD_SET_STRUCTURE_LABEL => Some(WebCommand::SetStructureLabel(take_string())),
                CMD_LOAD_DEFAULT_STRUCTURE => Some(WebCommand::LoadDefaultStructure),
                CMD_SET_THEME_INDEX => {
                    Some(WebCommand::SetThemeIndex(
                        unsafe { protdot_command_u32() } as usize
                    ))
                }
                _ => None,
            };

            unsafe {
                protdot_command_consume();
            }

            if let Some(command) = command {
                commands.push(command);
            }
        }

        commands
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use super::WebCommand;

    pub fn poll_commands() -> Vec<WebCommand> {
        Vec::new()
    }
}

pub use imp::poll_commands;
