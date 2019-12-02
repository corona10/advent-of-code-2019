pub struct Machine {}

pub fn create_machine() -> Machine {
    Machine {}
}

enum OPCODE {
    ADD = 1,
    MULT = 2,
    HALT = 99,
}

impl Machine {
    pub fn run(self, program: &Vec<i64>) -> i64 {
        let mut halt = false;
        let mut i = 0;
        let mut opcodes = program.clone();
        while !halt && i < opcodes.len() {
            let opcode = opcodes[i];
            if opcode == OPCODE::HALT as i64 {
                halt = true
            } else if opcode == OPCODE::ADD as i64 {
                let (one, two, pos) = (
                    opcodes[i + 1] as usize,
                    opcodes[i + 2] as usize,
                    opcodes[i + 3] as usize,
                );
                let (one_value, two_value) = (opcodes[one], opcodes[two]);
                opcodes[pos] = one_value + two_value;
                i += 4;
            } else if opcode == OPCODE::MULT as i64 {
                let (one, two, pos) = (
                    opcodes[i + 1] as usize,
                    opcodes[i + 2] as usize,
                    opcodes[i + 3] as usize,
                );
                let (one_value, two_value) = (opcodes[one], opcodes[two]);
                opcodes[pos] = one_value * two_value;
                i += 4;
            } else {
                panic!("Unexpected opcode: {}", opcode);
            }
        }
        opcodes[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::create_machine;

    #[test]
    fn test_example1() {
        let machine = create_machine();
        let opcodes = vec![1, 0, 0, 0, 99];
        let ret = machine.run(&opcodes);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_example2() {
        let machine = create_machine();
        let opcodes = vec![2, 3, 0, 3, 99];
        let ret = machine.run(&opcodes);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_example3() {
        let machine = create_machine();
        let opcodes = vec![2, 4, 4, 5, 99, 0];
        let ret = machine.run(&opcodes);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_example4() {
        let machine = create_machine();
        let opcodes = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let ret = machine.run(&opcodes);
        assert_eq!(ret, 30);
    }
}
