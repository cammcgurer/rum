use crate::data::Um;
use std::io::Read;

/// Moves the value in r[b] to r[a] if the value in the r[c] is not equal to 0
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'a': Value of bit field a
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn conditional_move(data: &mut Um, a: u32, b: u32, c: u32) {
    if data.registers[c as usize] != 0 {
        data.registers[a as usize] = data.registers[b as usize];
    }

    data.program_counter += 1;
}

/// Load the memory segment m[r[b][r[c]] in r[a]
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'a': Value of bit field a
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn segmented_load(data: &mut Um, a: u32, b: u32, c: u32) {
    data.registers[a as usize] = data.memory[data.registers[b as usize] as usize][data.registers[c as usize] as usize];

    data.program_counter += 1;
}

/// Store the value of r[c] in the memory segment m[r[a]][r[b]]
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'a': Value of bit field a
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn segmented_store(data: &mut Um, a: u32, b: u32, c: u32) {
    data.memory[data.registers[a as usize] as usize][data.registers[b as usize] as usize] = data.registers[c as usize];

    data.program_counter += 1;
}

/// Adds the value of r[b] to the value of r[c], mods by 2^32 to avoid overflow
/// and then stores the resulting value in r[a]
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'a': Value of bit field a
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn add(data: &mut Um, a: u32, b: u32, c: u32) {
    data.registers[a as usize] = data.registers[b as usize].wrapping_add(data.registers[c as usize]);

    data.program_counter += 1;
}

/// Multiplies the value of r[b] with the value of r[c], mods by 2^32 to avoid overflow
/// and then stores the resulting value in r[a]
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'a': Value of bit field a
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn multiply(data: &mut Um, a: u32, b: u32, c: u32) {
    data.registers[a as usize] = data.registers[b as usize].wrapping_mul(data.registers[c as usize]); 

    data.program_counter += 1;
}

/// Divides the value of r[b] by  the value of r[c] with integer division and then stores the resulting value in r[a]
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'a': Value of bit field a
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn divide(data: &mut Um, a: u32, b: u32, c: u32) {
    data.registers[a as usize] = data.registers[b as usize] / data.registers[c as usize];

    data.program_counter += 1;
}

/// Nands the bits from r[b] and r[c] then stores the value in r[a]
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'a': Value of bit field a
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn bitwise_nand(data: &mut Um, a: u32, b: u32, c: u32) {
    data.registers[a as usize] = !(data.registers[b as usize] & data.registers[c as usize]);

    data.program_counter += 1;
}

/// A new memory segment is created where the number of words is equal to the value in r[c], 
/// each word is initialized to 0, a bit pattern that is not all zeroes and does not identify 
/// any currently mapped segment is placed in r[b]. The new segment is mapped as m[r[b]]
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn map_segment(data: &mut Um, b: u32, c: u32) {
    if data.queue.len() >= 1 {
        data.registers[b as usize] = data.queue.pop().unwrap();
        data.memory[data.registers[b as usize] as usize] = vec![0; data.registers[c as usize] as usize];
    } else {
        data.memory.push(vec![0; data.registers[c as usize] as usize]);
        data.registers[b as usize] = (data.memory.len() - 1) as u32;
    }

    data.program_counter += 1;
}

/// Memory segment m[r[c]] is emptied but the vector is not removed from the 2d vector
/// The index of the unmapped memory segment is added to the queue vector
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'c': Value of bit field c
pub fn unmap_segment(data: &mut Um, c: u32) {
    data.memory[data.registers[c as usize] as usize].clear();
    data.queue.push(data.registers[c as usize]);

    data.program_counter += 1;

}

/// Prints r[c] to the terminal (must be value 0 - 255)
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'c': Value of bit field c
pub fn output(data: &mut Um, c: u32) {
    if data.registers[c as usize] > 255 { panic!() };
    print!("{}", char::from_u32(data.registers[c as usize]).unwrap());

    data.program_counter += 1;
}

/// Takes input from the terminal (must be value 0 - 255) 
/// and stores it in r[c] If the end of input has been signaled, then r[c] is loaded
/// with a full 32-bit word in which every bit is 1.
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'c': Value of bit field c
pub fn input(data: &mut Um, c: u32) {
    let mut buffer = [0; 1];
    let input = std::io::stdin().read(&mut buffer);
    match input {
        Ok (1) => {
            data.registers[c as usize] = buffer[0] as u32;
        } 
        Ok (_) => {
            data.registers[c as usize] = u32::MAX;
        }
        Err (_) => {
            panic!();
        } 
    } 

    data.program_counter += 1;
}

/// Replaces m[0] with m[r[b]] and sets our program counter to m[0][r[c]]
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'b': Value of bit field b
/// * 'c': Value of bit field c
pub fn load_program(data: &mut Um, b: u32, c: u32) {
    // if data.registers[b as usize] != 0 {
        data.memory[0] = data.memory[data.registers[b as usize] as usize].clone();
    // }

    data.program_counter = data.registers[c as usize] as usize;
}

/// Stores the value of the last 25 bits of the instruction word to the register
/// that corresponds to the three bits directly after the opcode
///
/// # Arguments:
/// * 'data': Mutable reference to Um data struct
/// * 'rl': Value of bit field after opcode
/// * 'vl': Value to load
pub fn load_value(data: &mut Um, rl: u32, vl: u32) {
    data.registers[rl as usize] = vl;

    data.program_counter += 1;
}
