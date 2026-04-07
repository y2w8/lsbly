use crate::disks::{Disks};

mod disks;

#[derive(Debug)]
pub enum LsblyError {
    CantRunLsblk,
    InvalidUtf8,
    JsonParseError,
}

fn main() -> Result<(), LsblyError> {
    println!("Hello, world!");

    let disks = Disks::list()?;

    // TODO: add size, used, available
    for disk in disks {
        println!("name: {}", disk.name.as_deref().unwrap_or_default());
        println!("path: {}", disk.path.unwrap().as_os_str().to_str().unwrap_or_default());
        println!("label: {}", disk.label.as_deref().unwrap_or_default());
        if let Some(child_list) = disk.children {
            for partition in child_list {
                println!("   name: {}", partition.name.as_deref().unwrap_or_default());
                println!("   path: {}", partition.path.unwrap().as_os_str().to_str().unwrap_or_default());
                println!("   label: {}", partition.label.as_deref().unwrap_or_default());
                println!("   ---");
            }
        }
        println!("---");
    }
    Ok(())
}
