// SPDX-License-Identifier: Apache-2.0

use crate::kvm::types::*;
use crate::launch::linux::ioctl::*;
use crate::launch::*;

use std::io::Result;
use std::os::unix::io::AsRawFd;

/// A new SNP-encrypted VM instance, one that was not previously running.
pub struct New;

/// An SNP-encrypted VM instance that has already been initialized.
pub struct Started;

/// Facilitates the correct execution of the V launch process.
pub struct Launcher<'a, T, U: AsRawFd, V: AsRawFd> {
    _state: T,
    kvm: &'a mut U,
    sev: &'a mut V,
}

impl<'a, U: AsRawFd, V: AsRawFd> Launcher<'a, New, U, V> {
    /// Begin the SEV-SNP launch process by creating a Launcher and issuing the
    /// KVM_SNP_INIT ioctl.
    pub fn new(kvm: &'a mut U, sev: &'a mut V) -> Result<Self> {
        let launcher = Launcher {
            _state: New,
            kvm,
            sev,
        };

        let init = Init { flags: 0 };

        let mut cmd = Command::from(launcher.sev, &init);
        SNP_INIT.ioctl(launcher.kvm, &mut cmd)?;

        Ok(launcher)
    }

    /// Initialize the flow to launch a guest.
    pub fn start(self, start: &mut Start) -> Result<Launcher<'a, Started, U, V>> {
        start.policy.flags |= PolicyFlags::RESERVED;
        let mut launch_start = LaunchStart::new(start);
        let mut cmd = Command::from_mut(self.sev, &mut launch_start);

        SNP_LAUNCH_START.ioctl(self.kvm, &mut cmd)?;

        let launcher = Launcher {
            _state: Started,
            kvm: self.kvm,
            sev: self.sev,
        };

        Ok(launcher)
    }
}
