use super::State;

#[test]
/// Add the contents of memory 0:50H (word data)
/// to contents of DW register, and store the result to 0:50H:
fn test_add () {
    let mut state = State::new();
    state.aw  = 0x1111;
    state.ds1 = 0x1112;

    state.program = vec![
        0xBA, 0x88, 0x88,  // MOV DW, 0x8888
        0xB8, 0x00, 0x00,  // MOV AW, 0x0000
        0xC4,              // MOV DS1, AW
        0xBF, 0x50, 0x00,  // MOV IY, 0x0050
        0x01, 0b00_010_101 // ADD DS1: WORD PTR [IY], DW
    ];

    state.step();

    assert_eq!(state.clock, 2);
    assert_eq!(state.pc, 3);
    assert_eq!(state.dw, 0x8888);

    state.step();

    assert_eq!(state.clock, 4);
    assert_eq!(state.pc, 6);
    assert_eq!(state.aw, 0x0000);

    state.step();

    assert_eq!(state.clock, 14);
    assert_eq!(state.pc, 7);
    assert_eq!(state.ds1, 0x0000);

    state.step();

    assert_eq!(state.clock, 16);
    assert_eq!(state.pc, 10);
    assert_eq!(state.iy, 0x0050);

    state.step();

    assert_eq!(state.clock, 23);
    assert_eq!(state.pc, 12);
    assert_eq!(state.memory[0x0050], 0x88);
    assert_eq!(state.memory[0x0051], 0x88);
}
