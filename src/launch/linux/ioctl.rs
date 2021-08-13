// SPDX-License-Identifier: Apache-2.0

use crate::impl_const_id;
use crate::kvm::types::*;
use iocuddle::*;

use std::marker::PhantomData;
use std::os::raw::c_ulong;
use std::os::unix::io::AsRawFd;

// These enum ordinal values are defined in the Linux kernel
// source code: include/uapi/linux/kvm.h
impl_const_id! {
    pub Id => u32;
    Init = 256,
    LaunchStart<'_> = 257,
    LaunchUpdate<'_> = 258,
    LaunchFinish<'_> = 259,
}

const KVM: Group = Group::new(0xAE);
const ENC_OP: Ioctl<WriteRead, &c_ulong> = unsafe { KVM.write_read(0xBA) };

// Note: the iocuddle::Ioctl::lie() constructor has been used here because
// KVM_MEMORY_ENCRYPT_OP ioctl was defined like this:
//
// _IOWR(KVMIO, 0xba, unsigned long)
//
// Instead of something like this:
//
// _IOWR(KVMIO, 0xba, struct kvm_sev_cmd)
//
// which would require extra work to wrap around the design decision for
// that ioctl.

/// Initialize the SEV-SNP platform in KVM.
pub const SNP_INIT: Ioctl<WriteRead, &Command<Init>> = unsafe { ENC_OP.lie() };

/// Initialize the flow to launch a guest.
pub const SNP_LAUNCH_START: Ioctl<WriteRead, &Command<LaunchStart>> = unsafe { ENC_OP.lie() };

/// Insert pages into the guest physical address space.
pub const SNP_LAUNCH_UPDATE: Ioctl<WriteRead, &Command<LaunchUpdate>> = unsafe { ENC_OP.lie() };

/// Complete the guest launch flow.
pub const SNP_LAUNCH_FINISH: Ioctl<WriteRead, &Command<LaunchFinish>> = unsafe { ENC_OP.lie() };

#[repr(C)]
pub struct Command<'a, T: Id> {
    code: u32,
    data: u64,
    error: u32,
    sev_fd: u32,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: Id> Command<'a, T> {
    pub fn from(sev: &'a mut impl AsRawFd, subcmd: &'a T) -> Self {
        Self {
            code: T::ID,
            data: subcmd as *const T as _,
            error: 0,
            sev_fd: sev.as_raw_fd() as _,
            _phantom: PhantomData,
        }
    }

    pub fn from_mut(sev: &'a mut impl AsRawFd, subcmd: &'a mut T) -> Self {
        Self {
            code: T::ID,
            data: subcmd as *mut T as _,
            error: 0,
            sev_fd: sev.as_raw_fd() as _,
            _phantom: PhantomData,
        }
    }
}
