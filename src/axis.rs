use enum_map::Enum;
use evdev::AbsoluteAxisType;


#[derive(Enum)]
pub enum Axis {
    LeftTrigger,
    RightTrigger,
    LeftJoystickX,
    LeftJoystickY,
    RightJoystickX,
    RightJoystickY,
    LeftTrackpadX,
    LeftTrackpadY,
    RightTrackpadX,
    RightTrackpadY,
    Unknown
}

impl From<AbsoluteAxisType> for Axis {
    fn from(axis_type: AbsoluteAxisType) -> Self {
        match axis_type {
            AbsoluteAxisType::ABS_X => Axis::LeftJoystickX,
            AbsoluteAxisType::ABS_Y => Axis::LeftJoystickY,
            AbsoluteAxisType::ABS_RX => Axis::RightJoystickX,
            AbsoluteAxisType::ABS_RY => Axis::RightJoystickY,
            AbsoluteAxisType::ABS_HAT0X => Axis::LeftTrackpadX,
            AbsoluteAxisType::ABS_HAT0Y => Axis::LeftTrackpadY,
            AbsoluteAxisType::ABS_HAT1X => Axis::RightTrackpadX,
            AbsoluteAxisType::ABS_HAT1Y => Axis::RightTrackpadY,
            AbsoluteAxisType::ABS_HAT2Y  => Axis::LeftTrigger,
            AbsoluteAxisType::ABS_HAT2X => Axis::RightTrigger,
            _ => Axis::Unknown
        }
    }
}
