use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardBindings {
  pub move_forward: String,
  pub move_backward: String,
  pub move_left: String,
  pub move_right: String,
  pub move_up: String,
  pub move_down: String,
  pub yaw_left: String,
  pub yaw_right: String,
  pub pitch_up: String,
  pub pitch_down: String,
  pub roll_left: String,
  pub roll_right: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ControlSource {
  LeftStick,
  RightStick,
  DPad,
  FaceButtons,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamepadBindings {
  pub move_horizontal: ControlSource,
  pub move_up: u16,
  pub move_down: u16,
  pub pitch_yaw: ControlSource,
  pub roll_left: u16,
  pub roll_right: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub ip: String,
  pub stream_port: u16,
  pub control_port: u16,
  pub keyboard: KeyboardBindings,
  pub gamepad: GamepadBindings,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      ip: "10.10.10.10".to_string(),
      stream_port: 8889,
      control_port: 5000,
      keyboard: KeyboardBindings {
        move_forward: "W".to_string(),
        move_backward: "S".to_string(),
        move_left: "A".to_string(),
        move_right: "D".to_string(),
        move_up: "Space".to_string(),
        move_down: "LShift".to_string(),
        pitch_up: "I".to_string(),
        pitch_down: "K".to_string(),
        yaw_left: "J".to_string(),
        yaw_right: "L".to_string(),
        roll_left: "Q".to_string(),
        roll_right: "E".to_string(),
      },
      gamepad: GamepadBindings {
        move_horizontal: ControlSource::LeftStick,
        move_up: 9,
        move_down: 10,
        pitch_yaw: ControlSource::RightStick,
        roll_left: 7,
        roll_right: 8,
      },
    }
  }
}
