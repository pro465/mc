use std::collections::HashMap;

fn main() {
    let mut a = std::env::args().skip(1);
    let s =
        std::fs::read_to_string(a.next().unwrap_or_else(help)).expect("could not read input file");

    let mnemonics = HashMap::from([
        ("halt", 0), // 0x0 0x00
        ("trap", 1), // 0x0 addr
        ("cmp", 2),  // reg 0x00
        ("puth", 3), // reg val
        ("putl", 4), // reg val
        ("brk", 5),
        // 0b0 cnd reg reg
        // 0b1 cnd imm8
        ("lea", 6),  // reg imm8
        ("ld", 7),   // reg 0x0 reg
        ("st", 8),   // reg 0x0 reg
        ("mv", 9),   // reg 0x0 reg
        ("not", 10), // reg 0x0 reg
        ("and", 11), // reg reg reg
        ("or", 12),  // reg reg reg
        ("ls", 13),  // reg reg reg
        ("rs", 14),  // reg reg reg
        ("add", 15), // reg reg reg
    ]);

    let mut labels = HashMap::new();
    let mut curr;
    let mut res = Vec::new();
    let mut lines = s
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("//") && l.split_whitespace().count() > 0);

    {
        let l = lines.next().unwrap();
        assert!(
            l.starts_with(".origin "),
            "could not find the .origin header at the start"
        );

        for x in l.split_whitespace().skip(1).take(2) {
            let num = parse_hex_from_str(x.as_bytes());
            res.push(num);
        }

        curr = u16::from_be_bytes([res[0], res[1]]);
    }

    for l in lines.clone() {
        if l.starts_with("'") {
            labels.insert(l, curr);
        } else {
            curr += if l.starts_with(".b ") {
                unescape_and_write_bytes::<false>(&mut res, l[3..].as_bytes()) / 2
            } else if l.starts_with(".offset ") {
                l[".offset ".len()..].parse().unwrap()
            } else {
                1
            }
        }
    }

    for l in lines.filter(|l| !l.starts_with("'")) {
        if l.starts_with('.') {
            if l.starts_with(".b ") {
                unescape_and_write_bytes::<true>(&mut res, l[3..].as_bytes());
            } else if l.starts_with(".offset ") {
                res.extend(
                    std::iter::repeat(0).take(l[".offset ".len()..].parse::<usize>().unwrap() * 2),
                );
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

fn instr(res: &mut Vec<u8>, labels: &HashMap<&str, u16>, mnemonics: &HashMap<&str, u16>, l: &str) {
    let l: Vec<_> = l.split_whitespace().collect();
    let mut encoding = mnemonics[l[0]] << 12;
    let check = |l: &str, bits: u32| {
        let x = dbg!(labels[l]);
        let x = x.wrapping_sub(u16::try_from(res.len() / 2).unwrap());

        let rem = x >> bits;

        if rem == 0 || rem == 0xffff >> bits {
            x & (1 << bits + 1) - 1
        } else {
            panic!("label {:?} is not located close enough", l);
        }
    };

    dbg!(&l);

    encoding |= match l[0] {
        "puth" if l[2].starts_with("'") => labels[l[2]] >> 8,
        "putl" if l[2].starts_with("'") => labels[l[2]] & 0xff,
        "lea" if l[2].starts_with("'") => check(l[2], 7),
        "brk" if l[2].starts_with("'") => 0x800 | check(l[2], 7),
        _ => l
            .get(2)
            .map_or(0, |n| parse_hex_from_str(n.as_bytes()).into()),
    };

    encoding |= l
        .get(1)
        .map_or(0, |l| u16::from(parse_hex(l.as_bytes()[0]).unwrap()))
        << 8;

    res.extend_from_slice(&encoding.to_be_bytes());
}

fn unescape_and_write_bytes<const PUSH: bool>(res: &mut Vec<u8>, escaped: &[u8]) -> u16 {
    let mut bytes = escaped.iter().enumerate();
    let mut bytes_len = 0;

    while let Some((i, b)) = bytes.next() {
        let b = match b {
            b'\\' => {
                let (b, size) = unescape(&escaped[i + 1..]);
                (0..size).for_each(|_| {
                    bytes.next();
                });
                b
            }
            _ => *b,
        };

        if PUSH {
            res.push(b);
        }

        bytes_len += 1;
    }

    if bytes_len & 1 > 0 {
        if PUSH {
            res.push(0);
        }

        bytes_len + 1
    } else {
        bytes_len
    }
}

fn unescape(s: &[u8]) -> (u8, usize) {
    match s[0] {
        b'x' => (parse_hex_from_str(&s[1..]), 3),
        b => (b, 1),
    }
}

fn parse_hex_from_str(x: &[u8]) -> u8 {
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
