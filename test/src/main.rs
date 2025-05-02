use anyhow::Result;
use pollster::block_on;
use teacup::run;

fn main() -> Result<()> {
    block_on(run())
}
