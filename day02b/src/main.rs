use std::fmt::Debug;

struct Policy {
    pos1: u8,
    pos2: u8,
    letter: u8,
}

impl Debug for Policy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = self.letter as char;
        f.debug_struct("Policy")
            .field("pos1", &self.pos1)
            .field("pos2", &self.pos2)
            .field("letter", &letter)
            .finish()
    }
}

struct Password<'a>(&'a [u8]);

impl<'a> Debug for Password<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let password = std::str::from_utf8(self.0).unwrap();
        f.debug_tuple("Password").field(&password).finish()
    }
}

#[derive(Debug)]
struct PasswordEntry<'a> {
    policy: Policy,
    password: Password<'a>,
}

struct PolicyError;

impl<'a> PasswordEntry<'a> {
    fn from_str(s: &'a str) -> Self {
        let s = s.as_bytes();
        let (pos1, tail) = {
            let dash_idx = s.iter().position(|c| *c == b'-').unwrap();
            s.split_at(dash_idx)
        };
        // tail[0] is '-'
        let tail = &tail[1..];
        let pos1: u8 = u8_slice_to_u8(pos1);
        let (pos2, tail) = {
            let space_idx = tail.iter().position(|c| *c == b' ').unwrap();
            tail.split_at(space_idx)
        };
        // tail[0] is ' '
        let tail = &tail[1..];
        let pos2: u8 = u8_slice_to_u8(pos2);
        let letter: u8 = tail[0];
        // tail[1] is ':'
        // tail[2] is ' '
        let password = &tail[3..];

        let password = Password(password);
        let policy = Policy {
            pos1: pos1,
            pos2: pos2,
            letter,
        };
        Self { policy, password }
    }

    fn validate(&self) -> Result<(), PolicyError> {
        let Self {
            policy: Policy { pos1, pos2, letter },
            password: Password(password),
        } = self;
        let matches_at_pos1 = password[*pos1 as usize - 1] == *letter;
        let matches_at_pos2 = password[*pos2 as usize - 1] == *letter;

        if matches_at_pos1 ^ matches_at_pos2 {
            Ok(())
        } else {
            Err(PolicyError)
        }
    }
}

fn u8_slice_to_u8(slice: &[u8]) -> u8 {
    slice
        .into_iter()
        .rev()
        .map(|ch| ch - b'0')
        .enumerate()
        .map(|(i, digit)| digit * 10u8.pow(i as u32))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string(r"C:\Users\USER\Documents\github\aoc2020\day02a\input.txt")
        .unwrap();
    let mut valid_count = 0;
    for line in input.lines() {
        println!("{line}");
        let entry = PasswordEntry::from_str(line);
        println!("{:?}", entry);
        if entry.validate().is_ok() {
            println!("Valid");
            valid_count += 1;
        } else {
            println!("Invalid");
        }
    }
    println!("Valid passwords: {valid_count}");
}
