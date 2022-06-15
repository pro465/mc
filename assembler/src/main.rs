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
    let mut res = Vec::new();
    let mut lines = s
        .lines()
        .filter(|l| !l.is_empty() && l.split_whitespace().count() > 0);

    {
        let l = lines.next().unwrap();
        assert!(
            l.starts_with(".origin "),
            "could not find the .origin header at the start"
        );

        for x in l.split_whitespace().skip(1).take(2) {
            let num = parse_hex_from_str(x);
            res.push(num);
        }
    }

    for l in lines.clone() {
        if l.starts_with("'") {
            labels.insert(&l[1..], curr);
        } else if l.starts_with('.') {
            curr += if l.starts_with(".b ") {
                unescape_and_write_bytes::<false>(&mut res, &l[3..])
            } else if l.starts_with(".offset ") {
                l[".offset ".len()..].parse().unwrap()
            } else {
                panic!()
            }
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

    for l in lines.filter(|l| !l.starts_with("'")) {
        if l.starts_with('.') {
            if l.starts_with(".b ") {
                unescape_and_write_bytes::<true>(&mut res, &l[3..]);
            } else if l.starts_with(".offset ") {
                res.extend(std::iter::repeat(0).take(l[".offset ".len()..].parse().unwrap()));
            } else {
                panic!();
            }
        } else {
            instr(&mut res, &labels, &mnemonics, l);
        }
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
            [parse_hex_from_str(l[2]), parse_hex_from_str(l[3])]
        });

        return;
    }

    let arg: u8 = if encoding != 0 {
        parse_hex(l[1].as_bytes()[0]).unwrap()
    } else {
        0
    };

    let first = (encoding << 4) | (arg & 0xf);
    res.push(first);

    if encoding > 4 {
        let next = parse_hex_from_str(l[2]);
        res.push(next);
    }
}

fn unescape_and_write_bytes<const PUSH: bool>(res: &mut Vec<u8>, escaped: &str) -> u16 {
    let mut bytes = escaped.bytes().enumerate();
    let mut len = 0;

    while let Some((i, b)) = bytes.next() {
        let b = match b {
            b'\\' => {
                let (b, size) = unescape(&escaped[i + 1..]);
                (0..size).for_each(|_| {
                    bytes.next();
                });
                b
            }
            _ => b,
        };

        if PUSH {
            res.push(b);
        } else {
            len += 1;
        }
    }

    len
}

fn unescape(s: &str) -> (u8, usize) {
    match s.bytes().next().unwrap() {
        b'x' => (parse_hex_from_str(&s[1..]), 3),
        b => (b, 1),
    }
}

fn parse_hex_from_str(x: &str) -> u8 {
    let x = x.as_bytes();
    let f = |x| parse_hex(x).unwrap();

    f(x[0]) * 0x10 | f(x[1])
}

fn parse_hex(x: u8) -> Result<u8, String> {
    if !(b'0'..=b'9').contains(&x) && !(b'a'..=b'f').contains(&(x | 0x20)) {
        return Err(format!("{:?} is not a valid hex digit", x));
    };

    Ok(if x <= b'9' {
        x - b'0'
    } else {
        10 + ((x | 0x20) - b'a')
    })
}
