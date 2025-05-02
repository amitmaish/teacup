use anyhow::Result;
use pollster::block_on;
use teacup::run;

fn main() -> Result<()> {
    println!("hello");
    block_on(run())
}
