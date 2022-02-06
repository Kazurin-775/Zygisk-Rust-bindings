use crate::binding::{RawApiTable, StateFlags, ZygiskOption};

pub struct ZygiskApi {
    inner: &'static RawApiTable,
}

impl ZygiskApi {
    /// Connect to a root companion process and get a Unix domain socket for IPC.
    ///
    /// This API only works in the `pre[XXX]Specialize` functions due to SELinux restrictions.
    ///
    /// The `pre[XXX]Specialize` functions run with the same privilege of zygote.
    /// If you would like to do some operations with superuser permissions, register a handler
    /// function that would be called in the root process with `zygisk_companion!(handler_func)`.
    /// Another good use case for a companion process is that if you want to share some resources
    /// across multiple processes, hold the resources in the companion process and pass it over.
    ///
    /// The root companion process is ABI aware; that is, when calling this function from a 32-bit
    /// process, you will be connected to a 32-bit companion process, and vice versa for 64-bit.
    ///
    /// Returns a file descriptor to a socket that is connected to the socket passed to your
    /// module's companion request handler. Returns -1 if the connection attempt failed.
    pub fn connect_companion(&self) -> i32 {
        self.inner
            .connect_companion
            .map(|func| func(self.inner.this))
            .unwrap_or(-1)
    }

    /// Get the file descriptor of the root folder of the current module.
    ///
    /// This API only works in the `pre[XXX]Specialize` functions.
    /// Accessing the directory returned is only possible in the `pre[XXX]Specialize` functions
    /// or in the root companion process (assuming that you sent the fd over the socket).
    /// Both restrictions are due to SELinux and UID.
    ///
    /// Returns -1 if errors occurred.
    pub fn get_module_dir(&self) -> i32 {
        self.inner
            .get_module_dir
            .map(|func| func(self.inner.this))
            .unwrap_or(-1)
    }

    /// Set various options for your module.
    /// Please note that this function accepts one single option at a time.
    /// Check [ZygiskOption] for the full list of options available.
    pub fn set_option(&self, option: ZygiskOption) {
        if let Some(func) = self.inner.set_option {
            func(self.inner.this, option);
        }
    }

    /// Get information about the current process.
    /// Returns bitwise-or'd [StateFlags] values.
    pub fn get_flags(&self) -> StateFlags {
        self.inner
            .get_flags
            .map(|func| func(self.inner.this))
            .map(|raw| StateFlags::from_bits(raw).expect("unsupported flag returned by Magisk"))
            .unwrap_or(StateFlags::empty())
    }
}

impl ZygiskApi {
    pub(crate) fn from_raw(inner: &'static RawApiTable) -> ZygiskApi {
        ZygiskApi { inner }
    }
}
