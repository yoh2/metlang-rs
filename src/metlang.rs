use std::io::{self, Read, Stdin, Stdout, Write};

use thiserror::Error;

pub type Program = Vec<Instruction>;

/// Brainf*ck commposed instruction
#[derive(Debug)]
pub enum Instruction {
    /// Add operand to the pointer.
    PAdd(isize),

    /// Add operand to the byte addressed by the pointer.
    DAdd(isize),

    /// Output the byte addressed by the pointer.
    Output,

    /// Read a byte to the memory addressed by the pointer.
    Input,

    /// Loop while the byte addressed by the pointer is not zero.
    UntilZero(Program),
}

/// Parse Error
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected end of source")]
    UnexpectedEndOfSource,
    #[error("unexpected end of loop")]
    UnexpectedEndOfLoop,
}

/// Runtime Error
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("memory index {0} out of bounds")]
    MemoryOutOfBound(isize),
    #[error("IO error")]
    IoError(std::io::Error),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Token {
    PInc,
    PDec,
    DInc,
    DDec,
    LoopHead,
    LoopEnd,
    Input,
    Output,
}

struct TokenDef {
    pattern: &'static str,
    token: Token,
}

const TOKEN_DEF: [TokenDef; 8] = [
    // 手抜き: 最長一致させるため長い方を先に置いてある
    TokenDef {
        pattern: "これになりたい",
        token: Token::Output,
    },
    TokenDef {
        pattern: "ポチった",
        token: Token::LoopHead,
    },
    TokenDef {
        pattern: "これすき",
        token: Token::Input,
    },
    TokenDef {
        pattern: "にゃうね",
        token: Token::PInc,
    },
    TokenDef {
        pattern: "にゃう",
        token: Token::DInc,
    },
    TokenDef {
        pattern: "にゃ？",
        token: Token::DDec,
    },
    TokenDef {
        pattern: "にゃん",
        token: Token::PDec,
    },
    TokenDef {
        pattern: "ねる",
        token: Token::LoopEnd,
    },
];

struct Source<'a> {
    source_str: &'a str,
    ungot_buf: Option<Token>,
}

impl<'a> Source<'a> {
    fn new(source_str: &'a str) -> Self {
        Self {
            source_str,
            ungot_buf: None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.ungot_buf.take() {
            return Some(token);
        }

        for (index, _) in self.source_str.char_indices() {
            for def in &TOKEN_DEF {
                if self.source_str[index..].starts_with(def.pattern) {
                    self.source_str = &self.source_str[index + def.pattern.len()..];
                    return Some(def.token);
                }
            }
        }
        None
    }

    fn unget_token(&mut self, token: Token) {
        assert!(self.ungot_buf.is_none());

        self.ungot_buf = Some(token);
    }
}

pub fn parse(source: &str) -> Result<Program, ParseError> {
    parse_internal(&mut Source::new(source), true)
}

fn parse_internal(source: &mut Source<'_>, top_level: bool) -> Result<Program, ParseError> {
    let mut program = Program::new();

    while let Some(token) = source.next_token() {
        match token {
            Token::PInc => push_padd(source, &mut program, 1),
            Token::PDec => push_padd(source, &mut program, -1),
            Token::DInc => push_dadd(source, &mut program, 1),
            Token::DDec => push_dadd(source, &mut program, -1),
            Token::Input => program.push(Instruction::Input),
            Token::Output => program.push(Instruction::Output),
            Token::LoopHead => program.push(Instruction::UntilZero(parse_internal(source, false)?)),
            Token::LoopEnd => {
                if top_level {
                    return Err(ParseError::UnexpectedEndOfLoop);
                } else {
                    return Ok(program);
                }
            }
        }
    }

    if top_level {
        Ok(program)
    } else {
        Err(ParseError::UnexpectedEndOfSource)
    }
}

fn push_padd(source: &mut Source<'_>, program: &mut Program, initial_operand: isize) {
    push_xadd(
        source,
        program,
        initial_operand,
        Token::PInc,
        Token::PDec,
        Instruction::PAdd,
    );
}

fn push_dadd(source: &mut Source<'_>, program: &mut Program, initial_operand: isize) {
    push_xadd(
        source,
        program,
        initial_operand,
        Token::DInc,
        Token::DDec,
        Instruction::DAdd,
    );
}

fn push_xadd<'a>(
    source: &mut Source<'a>,
    program: &mut Program,
    initial_operand: isize,
    plus: Token,
    minus: Token,
    gen: fn(isize) -> Instruction,
) {
    let mut operand = initial_operand;
    while let Some(token) = source.next_token() {
        if token == plus {
            operand += 1;
        } else if token == minus {
            operand -= 1;
        } else {
            source.unget_token(token);
            break;
        }
    }

    if operand != 0 {
        program.push(gen(operand));
    }
}

struct Runtime {
    memory: Vec<u8>,
    ptr: isize,
    stdin: Stdin,
    stdout: Stdout,
}

impl Runtime {
    fn new(memsize: usize) -> Self {
        Self {
            memory: vec![0; memsize],
            ptr: 0,
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }

    fn add_ptr(&mut self, operand: isize) -> Result<(), RuntimeError> {
        self.ptr += operand;
        Ok(())
    }

    fn add_data(&mut self, operand: isize) -> Result<(), RuntimeError> {
        let data = self.deref_mem_mut()?;
        *data = (*data as isize).wrapping_add(operand) as u8;
        Ok(())
    }

    fn input(&mut self) -> Result<(), RuntimeError> {
        let Self {
            memory, ptr, stdin, ..
        } = self;
        let data = Self::deref_raw_mem_mut(memory, *ptr)?;
        stdin
            .read_exact(std::slice::from_mut(data))
            .map_err(RuntimeError::IoError)
    }

    fn output(&mut self) -> Result<(), RuntimeError> {
        let Self {
            memory,
            ptr,
            stdout,
            ..
        } = self;
        let data = Self::deref_raw_mem_mut(memory, *ptr)?;
        stdout
            .write_all(std::slice::from_mut(data))
            .map_err(RuntimeError::IoError)
    }

    fn deref_mem_mut(&mut self) -> Result<&mut u8, RuntimeError> {
        Self::deref_raw_mem_mut(&mut self.memory, self.ptr)
    }

    fn deref_raw_mem_mut(memory: &mut [u8], ptr: isize) -> Result<&mut u8, RuntimeError> {
        if (ptr < 0) || (ptr >= memory.len() as isize) {
            Err(RuntimeError::MemoryOutOfBound(ptr))
        } else {
            Ok(&mut memory[ptr as usize])
        }
    }
}

pub const DEFAULT_MEMSIZE: usize = 30000;

pub fn run(program: &Program) -> Result<(), RuntimeError> {
    run_with_memsize(program, DEFAULT_MEMSIZE)
}

pub fn run_with_memsize(program: &Program, memsize: usize) -> Result<(), RuntimeError> {
    run_internal(program, &mut Runtime::new(memsize))
}

fn run_internal(program: &Program, runtime: &mut Runtime) -> Result<(), RuntimeError> {
    use self::Instruction::*;

    for inst in program {
        match inst {
            PAdd(operand) => runtime.add_ptr(*operand)?,
            DAdd(operand) => runtime.add_data(*operand)?,
            Input => runtime.input()?,
            Output => runtime.output()?,
            UntilZero(sub) => {
                while *runtime.deref_mem_mut()? != 0 {
                    run_internal(sub, runtime)?;
                }
            }
        }
    }
    Ok(())
}
