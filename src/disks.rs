use std::{path::PathBuf, process::Command};

use bytesize::ByteSize;
use color_eyre::{Result, eyre::Context};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockDevice {
    // like "nvme0n1"
    pub name: Option<String>,

    // like "/dev/nvme0n1"
    pub path: Option<PathBuf>,

    // like "/home"
    pub mountpoint: Option<PathBuf>,

    pub mountpoints: Option<Vec<PathBuf>>,

    // like "aaa123-bbb456-ccc789"
    pub uuid: Option<String>,

    pub vendor: Option<String>,

    pub label: Option<String>,

    pub children: Option<Vec<BlockDevice>>,

    pub partlabel: Option<String>,

    pub partflags: Option<String>,

    pub partuuid: Option<String>,

    pub size: Option<ByteSize>,

    pub fssize: Option<ByteSize>,

    pub fsused: Option<ByteSize>,

    pub fsavailable: Option<ByteSize>,
}

pub struct Disks();

// TODO: find library or using code not command.
impl Disks {
    pub fn list() -> Result<Vec<BlockDevice>> {
        let json_disks: serde_json::Value = self::Disks::run_lsblk()?;

        let mut disks: Vec<BlockDevice> = Vec::new();
        if let Some(device_list) = json_disks["blockdevices"].as_array() {
            for disk in device_list {
                let mut mountpoints: Vec<PathBuf> = Vec::new();
                if let Some(mountpoints_list) = disk["mountpoints"].as_array() {
                    for mountpoint in mountpoints_list {
                        mountpoints.push(PathBuf::from(mountpoint.as_str().unwrap_or_default()));
                    }
                }

                let mut children: Vec<BlockDevice> = Vec::new();
                if let Some(child_list) = disk["children"].as_array() {
                    for child in child_list {
                        let mut mountpoints: Vec<PathBuf> = Vec::new();
                        if let Some(mountpoints_list) = child["mountpoints"].as_array() {
                            for mountpoint in mountpoints_list {
                                mountpoints
                                    .push(PathBuf::from(mountpoint.as_str().unwrap_or_default()));
                            }
                        }
                        children.push(BlockDevice {
                            name: Some(child["name"].as_str().unwrap_or_default().to_string()),
                            path: Some(PathBuf::from(child["path"].as_str().unwrap_or_default())),
                            mountpoint: Some(PathBuf::from(
                                child["mountpoint"].as_str().unwrap_or_default(),
                            )),
                            mountpoints: Some(mountpoints),
                            uuid: Some(child["uuid"].as_str().unwrap_or_default().to_string()),
                            vendor: Some(child["vendor"].as_str().unwrap_or_default().to_string()),
                            label: Some(child["label"].as_str().unwrap_or_default().to_string()),
                            partlabel: Some(child["partlabel"].as_str().unwrap_or_default().to_string()),
                            partflags: Some(child["partflags"].as_str().unwrap_or_default().to_string()),
                            partuuid: Some(child["partuuid"].as_str().unwrap_or_default().to_string()),
                            size: Some(ByteSize::b(child["size"].as_u64().unwrap_or(0))),
                            fssize: Some(ByteSize::b(child["fssize"].as_u64().unwrap_or(0))),
                            fsavailable: Some(ByteSize::b(
                                child["fssize"].as_u64().unwrap_or(0)
                                    - child["fsused"].as_u64().unwrap_or(0),
                            )),
                            fsused: Some(ByteSize::b(child["fsused"].as_u64().unwrap_or(0))),
                            ..Default::default()
                        });
                    }
                }

                disks.push(BlockDevice {
                    name: Some(disk["name"].as_str().unwrap_or_default().to_string()),
                    path: Some(PathBuf::from(disk["path"].as_str().unwrap_or_default())),
                    mountpoint: Some(PathBuf::from(
                        disk["mountpoint"].as_str().unwrap_or_default(),
                    )),
                    mountpoints: Some(mountpoints),
                    uuid: Some(disk["uuid"].as_str().unwrap_or_default().to_string()),
                    vendor: Some(disk["vendor"].as_str().unwrap_or_default().to_string()),
                    label: Some(disk["label"].as_str().unwrap_or_default().to_string()),
                    size: Some(ByteSize::b(disk["size"].as_u64().unwrap_or(0))),
                    fssize: Some(ByteSize::b(disk["fssize"].as_u64().unwrap_or(0))),
                    fsavailable: Some(ByteSize::b(
                        disk["fssize"].as_u64().unwrap_or(0) - disk["fsused"].as_u64().unwrap_or(0),
                    )),
                    fsused: Some(ByteSize::b(disk["fsused"].as_u64().unwrap_or(0))),
                    children: Some(children),
                    ..Default::default()
                });
            }
        }
        Ok(disks)
    }

    pub fn run_lsblk() -> Result<serde_json::Value> {
        let output = Command::new("lsblk")
            .args([
                "-o",
                "NAME,MOUNTPOINT,PATH,SIZE,UUID,LABEL,PARTLABEL,PARTFLAGS,PARTUUID,FSSIZE,FSUSED",
                "-Jb",
            ])
            .output().context("failed to run lsblk")?;

        let stdout_str = String::from_utf8(output.stdout).context("failed to extract string from cmd output")?;

        let json_output: serde_json::Value =
            serde_json::from_str(&stdout_str).context("failed to parse cmd output")?;

        Ok(json_output)
    }
}
