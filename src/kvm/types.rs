// SPDX-Indentifier-License: Apache-2.0

/// Initialize the SEV-SNP platform in KVM.
#[repr(C, packed)]
pub struct Init {
    /// Reserved space, must be always set to 0 when issuing the ioctl.
    pub flags: u64,
}
