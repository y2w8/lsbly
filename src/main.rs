use crate::{app::App, disks::Disks};
use color_eyre::{Result, eyre::Context};

mod disks;
mod tui;
mod app;

fn main() -> Result<()> {
    color_eyre::install()?;

    let disks = Disks::list().context("failed to run disks")?;
    let mut app = App::new(disks);
    let mut terminal = tui::setup_terminal(&mut app)?;
    tui::run(&mut terminal, &mut app).context("failed to run tui")?;

    // TODO: add mountpoints, vendor, uuid && use ratatui and crossterm instead od raw output

    // for disk in disks {
    //     println!("name: {}", disk.name.as_deref().unwrap_or_default());
    //     println!("path: {}", disk.path.unwrap().as_os_str().to_str().unwrap_or_default());
    //     println!("label: {}", disk.label.as_deref().unwrap_or_default());
    //     println!("size: {}", disk.size.unwrap().display());
    //     println!("fssize: {}", disk.fssize.unwrap().display());
    //     println!("fsused: {}", disk.fsused.unwrap().display());
    //     println!("fsavailable: {}", disk.fsavailable.unwrap().display());
    //     if let Some(child_list) = disk.children {
    //         for partition in child_list {
    //             println!("   name: {}", partition.name.as_deref().unwrap_or_default());
    //             println!("   path: {}", partition.path.unwrap().as_os_str().to_str().unwrap_or_default());
    //             println!("   label: {}", partition.label.as_deref().unwrap_or_default());
    //             println!("   partlabel: {}", partition.partlabel.as_deref().unwrap_or_default());
    //             println!("   partflags: {}", partition.partflags.as_deref().unwrap_or_default());
    //             println!("   partuuid: {}", partition.partuuid.as_deref().unwrap_or_default());
    //             println!("   size: {}", partition.size.unwrap().display());
    //             println!("   fssize: {}", partition.fssize.unwrap().display());
    //             println!("   fsused: {}", partition.fsused.unwrap().display());
    //             println!("   fsavailable: {}", partition.fsavailable.unwrap().display());
    //             println!("   ---");
    //         }
    //     }
    //     println!("---");
    // }
    Ok(())
}
