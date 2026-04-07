use std::{path::PathBuf, process::Command};

use bytesize::ByteSize;

use crate::LsblyError;

#[derive(Debug, Default, PartialEq)]
pub struct BlockDevice {
    // like "nvme0n1"
    pub name: Option<String>,

    // like "/dev/nvme0n1"
    pub path: Option<PathBuf>,

    // like "/home"
    pub mountpoint: Option<PathBuf>,

    // like "/home"
    pub uuid: Option<String>,

    pub label: Option<String>,

    pub children: Option<Vec<BlockDevice>>,

    pub size: Option<ByteSize>,

    pub used: Option<ByteSize>,

    pub available: Option<ByteSize>,
}

pub struct Disks();


// TODO: find library or using code not command.
impl Disks {
    pub fn list() -> Result<Vec<BlockDevice>, LsblyError> {
        let json_disks: serde_json::Value = self::Disks::run_lsblk()?;

        let mut disks: Vec<BlockDevice> = Vec::new();
        if let Some(device_list) = json_disks["blockdevices"].as_array() {
            for disk in device_list {
                let mut children: Vec<BlockDevice> = Vec::new();
                if let Some(child_list) = disk["children"].as_array() {
                    for child in child_list {
                        children.push(BlockDevice {
                            name: Some(child["name"].as_str().unwrap_or_default().to_string()),
                            path: Some(PathBuf::from(child["path"].as_str().unwrap_or_default())),
                            mountpoint: Some(PathBuf::from(child["mountpoint"].as_str().unwrap_or_default())),
                            uuid: Some(child["uuid"].as_str().unwrap_or_default().to_string()),
                            label: Some(child["label"].as_str().unwrap_or_default().to_string()),
                            size: Some(ByteSize::b(child["fssize"].as_u64().unwrap_or(0))),
                            used: Some(ByteSize::b(child["fsused"].as_u64().unwrap_or(0))),
                            ..Default::default()
                        });
                    }
                }
                disks.push(BlockDevice {
                    name: Some(disk["name"].as_str().unwrap_or_default().to_string()),
                    path: Some(PathBuf::from(disk["path"].as_str().unwrap_or_default())),
                    mountpoint: Some(PathBuf::from(disk["mountpoint"].as_str().unwrap_or_default())),
                    uuid: Some(disk["uuid"].as_str().unwrap_or_default().to_string()),
                    label: Some(disk["label"].as_str().unwrap_or_default().to_string()),
                    size: Some(ByteSize::b(disk["fssize"].as_u64().unwrap_or(0))),
                    available: Some(ByteSize::b(disk["fssize"].as_u64().unwrap_or(0) - disk["fsused"].as_u64().unwrap_or(0))),
                    used: Some(ByteSize::b(disk["fsused"].as_u64().unwrap_or(0))),
                    children: Some(children),
                });
            }
        }
        Ok(disks)
    }

    pub fn run_lsblk() -> Result<serde_json::Value, LsblyError> {
        let output = Command::new("lsblk")
            .args([
                "-o",
                "NAME,MOUNTPOINT,PATH,SIZE,UUID,LABEL,FSSIZE,FSUSED",
                "-Jb",
            ])
            .output()
            .map_err(|_| LsblyError::CantRunLsblk)?;

        let stdout_str = String::from_utf8(output.stdout).map_err(|_| LsblyError::InvalidUtf8)?;

        let json_output: serde_json::Value =
            serde_json::from_str(&stdout_str).map_err(|_| LsblyError::JsonParseError)?;

        Ok(json_output)
    }
}
