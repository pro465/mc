use std::collections::HashMap;

fn main() {
    let mut a = std::env::args().skip(1);
    let s =
        std::fs::read_to_string(a.next().unwrap_or_else(help)).expect("could not read input file");

    let mnemonics = HashMap::from([
        ("halt", 0),
        ("cmp", 1),  // reg
        ("trap", 2), // 4addr
        ("if", 3),   // 0 cnd
        ("push", 4), // reg
        ("put", 5),  // reg val
        ("ld", 6),   // reg reg reg
        ("st", 7),   // reg reg reg
        ("or", 8),   // reg reg reg
        ("and", 9),  // reg reg reg
        ("xor", 10), // reg reg reg
        ("ls", 11),  // reg reg reg
        ("rs", 12),  // reg reg reg
        ("add", 13), // reg reg reg
        ("mv", 14),  // reg 0x0 reg
        ("jmp", 15),
        // 0b00 addrh addrl
        // 0b01 imm8
        // 0b10 0x00 reg
        // 0b11 reg reg
    ]);

    let mut labels = HashMap::new();
    let mut curr = 0;

    for l in s.lines() {
        if l.is_empty() || l.split_whitespace().count() == 0 {
            continue;
        }

        if l.starts_with("'") {
            labels.insert(&l[1..], curr);
        } else {
            let mut l = l.split_whitespace();
            let encoding = mnemonics[l.next().unwrap()];
            curr += if encoding < 5 {
                1
            } else {
                if encoding == 15 && l.next().unwrap() == "0" {
                    3
                } else {
                    2
                }
            };
        }
    }
    let mut res = vec![0; 2];

    for l in s
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with("'") && l.split_whitespace().count() > 0)
    {
        instr(&mut res, &labels, &mnemonics, l);
    }

    std::fs::write(a.next().unwrap_or_else(help), res).unwrap();
}

fn help() -> String {
    println!("usage: assembler <in_file> <out_file>");
    std::process::exit(-1);
}

fn instr(
    res: &mut Vec<u8>,
    labels: &HashMap<&str, u16>,
    mnemonics: &HashMap<&'static str, u8>,
    l: &str,
) {
    let l: Vec<_> = l.split_whitespace().collect();
    let encoding = mnemonics[l[0]];

    if encoding == 15 && l[1] == "0" {
        let first = encoding << 4;

        res.push(first);

        res.extend(if l[2].starts_with("'") {
            let idx = labels[&l[2][1..]];

            idx.to_be_bytes()
        } else {
            [l[2].parse().unwrap(), l[3].parse().unwrap()]
        });

        return;
    }

    let arg: u8 = if encoding != 0 {
        l[1].parse().unwrap()
    } else {
        0
    };
    let first = (encoding << 4) | (arg & 0xf);

    res.push(first);

    if encoding > 4 {
        let next = l[2].parse().unwrap();

        let next = if l.len() > 3 {
            (next << 4) | (l[3].parse::<u8>().unwrap() & 0xf)
        } else {
            next
        };

        res.push(next);
    }
}
