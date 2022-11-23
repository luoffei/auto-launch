use crate::{AutoLaunch, Result};
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

const AL_REGKEY: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
const AL_PRIVILEGES_REGEKEY: &str = "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run";

/// Windows implement
impl AutoLaunch {
    /// Create a new AutoLaunch instance
    /// - `app_name`: application name
    /// - `app_path`: application path
    /// - `args`: startup args passed to the binary
    ///
    /// ## Notes
    ///
    /// The parameters of `AutoLaunch::new` are different on each platform.
    pub fn new(app_name: &str, app_path: &str, args: &[impl AsRef<str>], elevate_privileges: bool) -> AutoLaunch {
        AutoLaunch {
            app_name: app_name.into(),
            app_path: app_path.into(),
            args: args.iter().map(|s| s.as_ref().to_string()).collect(),
            elevate_privileges,
        }
    }

    /// Enable the AutoLaunch setting
    ///
    /// ## Errors
    ///
    /// - failed to open the registry key
    /// - failed to set value
    pub fn enable(&self) -> Result<()> {
        let (regkey, keypath) = if self.elevate_privileges {
            (RegKey::predef(HKEY_LOCAL_MACHINE), AL_PRIVILEGES_REGEKEY)
        } else {
            (RegKey::predef(HKEY_CURRENT_USER), AL_REGKEY)
        };
        regkey.open_subkey_with_flags(keypath, KEY_SET_VALUE)?
            .set_value::<_, _>(
                &self.app_name,
                &format!("{} {}", &self.app_path, &self.args.join(" ")),
            )?;
        Ok(())
    }

    /// Disable the AutoLaunch setting
    ///
    /// ## Errors
    ///
    /// - failed to open the registry key
    /// - failed to delete value
    pub fn disable(&self) -> Result<()> {
        let (regkey, keypath) = if self.elevate_privileges {
            (RegKey::predef(HKEY_LOCAL_MACHINE), AL_PRIVILEGES_REGEKEY)
        } else {
            (RegKey::predef(HKEY_CURRENT_USER), AL_REGKEY)
        };
        regkey.open_subkey_with_flags(keypath, KEY_SET_VALUE)?
            .delete_value(&self.app_name)?;
        Ok(())
    }

    /// Check whether the AutoLaunch setting is enabled
    pub fn is_enabled(&self) -> Result<bool> {
        let (regkey, keypath) = if self.elevate_privileges {
            (RegKey::predef(HKEY_LOCAL_MACHINE), AL_PRIVILEGES_REGEKEY)
        } else {
            (RegKey::predef(HKEY_CURRENT_USER), AL_REGKEY)
        };
        Ok(regkey
            .open_subkey_with_flags(keypath, KEY_READ)?
            .get_value::<String, _>(&self.app_name)
            .is_ok())
    }
}
