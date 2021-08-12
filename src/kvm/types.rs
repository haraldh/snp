// SPDX-Indentifier-License: Apache-2.0

use crate::launch::*;

use std::marker::PhantomData;

/// Initialize the SEV-SNP platform in KVM.
#[repr(C, packed)]
pub struct Init {
    /// Reserved space, must be always set to 0 when issuing the ioctl.
    pub flags: u64,
}

/// Initialize the flow to launch a guest.
#[repr(C)]
pub struct LaunchStart<'a> {
    /// Guest policy. See Table 7 of the AMD SEV-SNP Firmware
    /// specification for a description of the guest policy structure.
    policy: u64,

    /// Bits 63:12 of the sPA of the guest context of the migration agent.
    /// Ignored if MA_EN is 0.
    ma_uaddr: u64,

    /// 1 if this guest is associated with a migration agent. Otherwise 0.
    ma_en: u8,

    /// Indicates that this launch flow is launching an IMI for the purpose of
    /// guest-assisted migration.
    imi_en: u8,

    /// Hypervisor provided value to indicate guest OS visible workarounds.
    /// The format is hypervisor defined.
    gosvw: [u8; 16],

    _phantom: PhantomData<&'a ()>,
}

impl<'a> LaunchStart<'a> {
    pub fn new(start: &'a Start) -> Self {
        Self {
            policy: start.policy.to_u64(),
            ma_uaddr: start.ma_uaddr,
            ma_en: start.ma_en,
            imi_en: start.imi_en,
            gosvw: start.gosvw,
            _phantom: PhantomData,
        }
    }
}

/// Insert pages into the guest physical address space.
#[repr(C)]
pub struct LaunchUpdate<'a> {
    /// Bits 63:12 of the sPA of the destination page. The page size
    /// is determined by PAGE_SIZE.
    uaddr: u64,

    /// Length of the
    len: u32,

    /// Indicates that this page is part of the IMI of the guest.
    imi_page: u8,

    /// Encoded page type. See Table 58 if the SNP Firmware specification.
    page_type: u8,

    /// VMPL permission mask for VMPL3. See Table 59 of the SNP Firmware
    /// specification for the definition of the mask.
    vmpl3_perms: u8,

    /// VMPL permission mask for VMPL2.
    vmpl2_perms: u8,

    /// VMPL permission mask for VMPL1.
    vmpl1_perms: u8,

    _phantom: PhantomData<&'a ()>,
}

impl<'a> LaunchUpdate<'a> {
    pub fn new(data: &'a [u8], update: &'a Update) -> Self {
        Self {
            uaddr: data.as_ptr() as _,
            len: data.len() as _,
            imi_page: update.imi_page,
            page_type: update.page_type.value(),
            vmpl3_perms: update.vmpl3_perms,
            vmpl2_perms: update.vmpl2_perms,
            vmpl1_perms: update.vmpl1_perms,
            _phantom: PhantomData,
        }
    }
}
