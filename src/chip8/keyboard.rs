use super::cpu::CPUState;

// Original keyboard layout:
// 1  2  3  C
// 4  5  6  D
// 7  8  9  E
// A  0  B  F
pub type KeyID = u8;

const KeyIDCount: u8 = 16;

pub fn is_key_pressed(state: &CPUState, key: KeyID) -> bool
{
    assert!(key < KeyIDCount); // Invalid key

    return (state.key_state & (1 << key)) != 0;
}

// If multiple keys are pressed at the same time, only register one.
pub fn get_key_pressed(key_state: u16) -> KeyID
{
    assert!(key_state != 0);

    for i in 1..16 {
        if ((1 << i) & key_state) != 0 {
            return i;
        }
    }

    assert!(false);
    return 0x0;
}

pub fn set_key_pressed(state: &mut CPUState, key: KeyID, pressed_state: bool)
{
    assert!(key < KeyIDCount); // Invalid key

    let key_mask: u16 = 1 << key;
    state.key_state = (state.key_state & !key_mask) | if pressed_state { key_mask } else { 0 };
}
