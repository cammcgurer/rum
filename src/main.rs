pub mod data;
pub mod funcs;
use std::env;

/// Prints out the name of the current instruction to be executed (used for debugging)
///
/// # Arguments:
/// * 'opcode': The bits of the opcode for the instruction
fn _opcode_name(opcode: u32) -> String {
    match opcode {
        0 => "CMove".to_string(),
        1 => "SegLoad".to_string(),
        2 => "SegStore".to_string(),
        3 => "Add".to_string(),
        4 => "Mult".to_string(),
        5 => "Div".to_string(),
        6 => "BitNand".to_string(),
        7 => "Halt".to_string(),
        8 => "MapSeg".to_string(),
        9 => "UnmapSeg".to_string(),
        10 => "Output".to_string(),
        11 => "Input".to_string(),
        12 => "LoadProg".to_string(),
        13 => "LoadVal".to_string(),
        opcode => format!("Unrecognized opcode {opcode}")
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let mut um = data::Um::new();
    um.get_instructions(&filename);

    // let mut count = 0;
    while um.program_counter < um.memory[0].len() {
        // if count == 50000000 {
        //     break
        // }
        // println!("executing: {}", _opcode_name(my_um.memory[0][my_um.program_counter] >> 28));
        um.execute(um.memory[0][um.program_counter]);
        // println!("{count}");
    }
}
