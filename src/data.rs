use crate::funcs;
use std::io::Read;
use std::process::exit;

pub struct Um {
    pub registers: Vec<u32>,
    pub memory: Vec<Vec<u32>>,
    pub program_counter: usize, 
    pub queue: Vec<u32>
}

impl Um {
    pub fn new() -> Self {
        Um {
            registers: vec![0; 8],
            memory: vec![],
            program_counter: 0,
            queue: vec![],
        }
    }

    /// Reads in the intructions and pushes the vector to memory
    ///
    /// # Arguments:
    /// * 'filename': reference string of the filename to read
    pub fn get_instructions(&mut self, filename: &str) {
        let mut raw_reader = std::fs::File::open(filename).unwrap();
        let mut buf = Vec::<u8>::new();
        raw_reader.read_to_end(&mut buf).unwrap();
        
        self.memory.push(buf
            .chunks_exact(4)
            .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
            .collect());
    }

    /// Defines which bits to keep when getting a value 
    ///
    /// # Arguments:
    /// * 'bits': bits to keep
    fn mask(&self, bits: u32) -> u32 { (1 << bits) - 1 }

    /// Gets the value from a bit field
    ///
    /// # Arguments:
    /// * 'field': reference to Field struct
    /// * 'inst': current instruction word
    fn get(&self, field: &Field, inst: u32) -> u32 {
        (inst >> field.lsb) & self.mask(field.width)
    }

    /// Executes the current instruction
    ///
    /// # Arguments:
    /// * 'inst': currrent instruction word
    pub fn execute(&mut self, inst: u32) {
        enum Opcode {
            CMove,
            SegLoad,
            SegStore,
            Add,
            Mult,
            Div,
            BitNand,
            Halt,
            MapSeg,
            UnmapSeg,
            Output,
            Input,
            LoadProg,
            LoadVal,
        }

        match self.get(&OP, inst) {
            o if o == Opcode::CMove as u32 => {
                funcs::conditional_move(self, self.get(&RA, inst), 
                    self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::SegLoad as u32 => {
                funcs::segmented_load(self, self.get(&RA, inst), 
                self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::SegStore as u32 => {
                funcs::segmented_store(self, self.get(&RA, inst), 
                self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::Add as u32 => {
                funcs::add(self, self.get(&RA, inst), 
                self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::Mult as u32 => {
                funcs::multiply(self, self.get(&RA, inst), 
                self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::Div as u32 => {
                funcs::divide(self, self.get(&RA, inst), 
                self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::BitNand as u32 => {
                funcs::bitwise_nand(self, self.get(&RA, inst), 
                self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::Halt as u32 => {
                exit(0);
            },
            o if o == Opcode::MapSeg as u32 => {
                funcs::map_segment(self, self.get(&RB, inst), 
                self.get(&RC, inst))
            },
            o if o == Opcode::UnmapSeg as u32 => {
                funcs::unmap_segment(self, self.get(&RC, inst))
            },
            o if o == Opcode::Output as u32 => {
                funcs::output(self, self.get(&RC, inst))
            },
            o if o == Opcode::Input as u32 => {
                funcs::input(self, self.get(&RC, inst))
            },
            o if o == Opcode::LoadProg as u32 => {
                funcs::load_program(self, self.get(&RB, inst), self.get(&RC, inst))
            },
            o if o == Opcode::LoadVal as u32 => {
                funcs::load_value(self, self.get(&RL, inst), self.get(&VL, inst))
            },
            _ => {
                panic!()
            }
        }
    }
}

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};
