use crossterm::{
    cursor,
    style::{self, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

use crate::simulator::Interperter;
use anyhow::Result;

const INSTRUCTIONS_PADDING: u16 = 2;
const STACK_PADDING: u16 = 64;

pub struct Debugger {
    timeout: u64,
    page_size: u16,
    stdout: Stdout,
}

impl Debugger {
    pub fn new(timeout: u64) -> Result<Self> {
        Ok(Self {
            timeout,
            page_size: 32,
            stdout: stdout(),
        })
    }

    pub fn start(&mut self) -> Result<()> {
        self.stdout.execute(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn show(&mut self, ctx: &Interperter) -> Result<()> {
        self.stdout.queue(Clear(ClearType::All))?;
        self.instructions(ctx)?;
        self.stack(ctx)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn timeout(&self) {
        use std::thread;
        use std::time::Duration;
        thread::sleep(Duration::from_millis(self.timeout));
    }

    fn instructions(&mut self, ctx: &Interperter) -> Result<()> {
        let actual_addr = ctx.addr() - 1;
        let instructions = ctx
            .code()
            .into_iter()
            .enumerate()
            .skip(actual_addr)
            .take(self.page_size as usize);
        for (y, (addr, instruction)) in instructions.enumerate() {
            self.stdout
                .queue(cursor::MoveTo(INSTRUCTIONS_PADDING, y as u16))?
                .queue(style::PrintStyledContent(if addr == actual_addr {
                    format!("{addr:0>16}\t{instruction:?}").negative()
                } else {
                    format!("{addr:0>16}\t{instruction:?}").reset()
                }))?;
        }
        Ok(())
    }

    fn stack(&mut self, ctx: &Interperter) -> Result<()> {
        for (y, value) in ctx.stack().into_iter().rev().enumerate() {
            self.stdout
                .queue(cursor::MoveTo(STACK_PADDING, y as u16))?
                .queue(PrintStyledContent(
                    format!("{}\t{value}", if y == 0 { '^' } else { '|' }).reset(),
                ))?;
        }
        Ok(())
    }
}

impl Drop for Debugger {
    fn drop(&mut self) {
        // Lol?
        let _ = self.stdout.execute(LeaveAlternateScreen);
    }
}
