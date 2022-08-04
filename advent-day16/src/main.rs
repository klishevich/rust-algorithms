use std::fs;
use std::i64;
use std::str;

struct PacketParser {
    ver_sum: u32,
    depth: u32,
}

// RECURSION
impl PacketParser {
    // REFERENCE TO MUTABLE INT
    fn read_int(bytes_str: &[u8], start: &mut usize, chars: usize) -> u32 {
        let end = *start + chars;
        let str = str::from_utf8(&bytes_str[*start..end]).unwrap();
        let val = u32::from_str_radix(str, 2).unwrap();
        *start += chars;
        return val;
    }

    fn read_move(start: &mut usize, chars: usize) {
        *start += chars;
    }
    // DISCUSS!!
    // fn read_str(bytes_str: &[u8], start: &mut usize, chars: usize) -> &str {
    //     let end = *start + chars;
    //     let str = str::from_utf8(&bytes_str[*start..end]).unwrap();
    //     *start += chars;
    //     return str;
    // }

    pub fn run(&mut self, s: &str) -> (usize, u32) {
        self.depth += 1;
        println!("-- PacketParser depth {} --", self.depth);
        println!("str {}", s);
        let mut pos: usize = 0;
        let bs = s.as_bytes();
        let p_ver = PacketParser::read_int(bs, &mut pos, 3);
        println!("ver {}", p_ver);
        self.ver_sum += p_ver;
        let p_type = PacketParser::read_int(bs, &mut pos, 3);
        println!("type {}", p_type);

        if p_type == 4 {
            println!("LITERAL");
            let mut accum_val: String = "".to_string();
            // LOOP
            loop {
                let first_bit = PacketParser::read_int(bs, &mut pos, 1);
                accum_val += str::from_utf8(&bs[pos..pos + 4]).unwrap();
                PacketParser::read_move(&mut pos, 4);
                if first_bit == 0 {
                    break;
                }
            }

            // println!("literal end pos {}, accum_val {}", pos, accum_val);
            let val = u32::from_str_radix(&accum_val, 2).unwrap();
            println!("val {}", val);
            self.depth -= 1;
            return (pos, val);
        } else {
            let length_id = PacketParser::read_int(bs, &mut pos, 1);
            if length_id == 0 {
                let bits_in_sub_packets: usize = PacketParser::read_int(bs, &mut pos, 15).try_into().unwrap();
                println!("BITS_IN_SUB_PACKETS {}", bits_in_sub_packets);
                let end_pos = pos + bits_in_sub_packets;
                println!("end_pos {}", end_pos);
                let mut end_sum = 0;
                loop {
                    println!("-loop depth {}", self.depth);
                    let sub_str = str::from_utf8(&bs[pos..s.len()]).unwrap();
                    let (r_pos, r_sum) = PacketParser::run(self, sub_str);
                    pos = pos + r_pos;
                    end_sum += r_sum;
                    println!("new pos {}", pos);
                    if pos >= end_pos {
                        break;
                    }
                }
                self.depth -= 1;
                return (end_pos, end_sum);
            } else {
                let sub_packets = PacketParser::read_int(bs, &mut pos, 11);
                println!("NUMBER_OF_SUB_PACKETS {}", sub_packets);
                let mut end_sum = 0;
                for _ in 0..sub_packets {
                    let sub_str = str::from_utf8(&bs[pos..s.len()]).unwrap();
                    let (r_pos, r_sum) = PacketParser::run(self, sub_str);
                    pos = pos + r_pos;
                    end_sum += r_sum;
                }
                self.depth -= 1;
                return (pos, end_sum);
            }
        }
    }
}

fn task(file_name: &str) -> (u32, u32) {
    let content = fs::read_to_string(file_name).expect("reading file error");
    let str = content.lines().next().unwrap();
    println!("str {}", str);
    // maybe string of bytes b""?
    let mut b_str = "".to_string();
    for c in str.chars() {
        let a = i64::from_str_radix(&c.to_string(), 16).unwrap();
        let b = format!("{a:b}");
        let mut b_str2 = "".to_string();
        let add_zeroes = 4 - b.chars().count();
        for _a in 0..add_zeroes {
            b_str2 += "0";
        }
        b_str2 += &b;
        // println!("b_str2 {}", b_str2);
        b_str += &b_str2;
    }

    let mut parser = PacketParser {
        ver_sum: 0,
        depth: 0,
    };
    let (_, end_sum) = parser.run(&b_str);
    println!("ver_sum {}", parser.ver_sum);
    println!("end_sum {}", end_sum);
    return (parser.ver_sum, end_sum);
}

fn main() {
    task("src/data21.txt");
}


mod test {
    use crate::task;

    #[test]
    fn test01() {
        let (ver, _) = task("src/data01.txt");
        assert_eq!(ver, 6);
    }
    #[test]
    fn test02() {
        let (ver, _) = task("src/data02.txt");
        assert_eq!(ver, 9);
    }
    #[test]
    fn test03() {
        let (ver, _) = task("src/data03.txt");
        assert_eq!(ver, 14);
    }
    #[test]
    fn test11() {
        let (ver, _) = task("src/data11.txt");
        assert_eq!(ver, 16);
    }
    #[test]
    fn test12() {
        let (ver, _) = task("src/data12.txt");
        assert_eq!(ver, 12);
    }
    #[test]
    fn test13() {
        let (ver, _) = task("src/data13.txt");
        assert_eq!(ver, 23);
    }
    #[test]
    fn test14() {
        let (ver, _) = task("src/data14.txt");
        assert_eq!(ver, 31);
    }
    #[test]
    fn test21() {
        let (_, sum) = task("src/data21.txt");
        assert_eq!(sum, 3);
    }
    #[test]
    fn test22() {
        let (_, product) = task("src/data22.txt");
        assert_eq!(product, 54);
    }
}