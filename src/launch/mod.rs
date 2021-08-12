// SPDX-License-Identifier: Apache-2.0

/// Launcher API
pub mod launcher;

#[cfg(target_os = "linux")]
mod linux;

use super::*;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Configurable SNP Policy options.
    #[derive(Default, Deserialize, Serialize)]
    pub struct PolicyFlags: u16 {
        /// Enable if SMT is enabled in the host machine.
        const SMT =         0b0000000000000001;

        /// Do not use: This value will always be enabled (set to 1) when calling Launcher::start().
        const RESERVED =    0b0000000000000010;

        /// If enabled, association with a migration agent is allowed.
        const MIGRATE_MA =  0b0000000000000100;

        /// If enabled, debugging is allowed.
        const DEBUG =       0b0000000000001000;
    }
}

impl PolicyFlags {
    /// Represent policy flags to their u16 counterpart.
    pub fn to_u16(&self) -> u16 {
        let mut val: u16 = 0;

        let smt = PolicyFlags::SMT;
        let migrate_ma = PolicyFlags::MIGRATE_MA;
        let debug = PolicyFlags::DEBUG;

        if *self & smt == smt {
            val |= 0b1;
        }
        if *self & migrate_ma == migrate_ma {
            val |= 0b100;
        }
        if *self & debug == debug {
            val |= 0b1000;
        }

        val |= 0b10;

        val
    }
}

/// Describes a policy that the AMD Secure Processor will
/// enforce.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Policy {
    /// The various policy optons are encoded as bit flags.
    pub flags: PolicyFlags,

    /// The desired minimum platform firmware version.
    pub minfw: Version,
}

impl Policy {
    /// Convert a Policy to it's u64 counterpart.
    pub fn to_u64(&self) -> u64 {
        let mut val: u64 = 0;

        let minor_version = u64::from(self.minfw.minor);
        let mut major_version = u64::from(self.minfw.major);
        let flags: u16 = self.flags.to_u16();
        let mut flags_64: u64 = u64::from(flags);

        major_version <<= 8;
        flags_64 <<= 16;

        val |= minor_version;
        val |= major_version;
        val |= flags_64;
        val &= 0x00FFFFFF;

        val
    }
}

/// Encapsulates the various data needed to begin the launch process.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Start {
    /// Describes a policy that the AMD Secure Processor will enforce.
    pub policy: Policy,

    /// Bits 63:12 of the sPA of the guest context of the migration agent. Ignored if MA_EN is 0.
    pub ma_uaddr: u64,

    /// 1 if this guest is associated with a migration agent. Otherwise 0.
    pub ma_en: u8,

    /// Indicates that this launch flow is launching an IMI for the purpose of guest-assisted migration.
    pub imi_en: u8,

    /// Hypervisor provided value to indicate guest OS visible workarounds.The format is hypervisor defined.
    pub gosvw: [u8; 16],
}

/// Encapsulates the various data needed to begin the update process.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Update {
    /// Indicates that this page is part of the IMI of the guest.
    pub imi_page: u8,

    /// Encoded page type.
    pub page_type: PageType,

    /// VMPL3 permission mask.
    pub vmpl3_perms: u8,

    /// VMPL2 permission mask.
    pub vmpl2_perms: u8,

    /// VMPL1 permission mask.
    pub vmpl1_perms: u8,
}

/// Encoded page types for a launch update. See Table 58 of the SNP Firmware
/// specification for further details.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum PageType {
    /// A normal data page.
    Normal,

    /// A VMSA page.
    Vmsa,

    /// A page full of zeroes.
    Zero,

    /// A page that is encrypted but not measured
    Unmeasured,

    /// A page for the firmware to store secrets for the guest.
    Secrets,

    /// A page for the hypervisor to provide CPUID function values.
    Cpuid,
}

impl PageType {
    /// Get the encoded value for a page type. See Table 58 of the SNP
    /// Firmware specification for further details.
    pub fn value(self) -> u8 {
        match self {
            PageType::Normal => 0x1,
            PageType::Vmsa => 0x2,
            PageType::Zero => 0x3,
            PageType::Unmeasured => 0x4,
            PageType::Secrets => 0x5,
            PageType::Cpuid => 0x6,
        }
    }
}
