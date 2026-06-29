//! DLL Loader Module
//!
//! Provides functionality to dynamically load DLLs and call functions within them

use crate::error::{FrameworkError, FrameworkResult};
use libloading::{Library, Symbol};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use parking_lot::RwLock;

/// DLL Loader
pub struct DllLoader {
    /// Loaded DLL libraries
    libraries: RwLock<HashMap<String, Library>>,
    /// DLL search paths
    search_paths: RwLock<Vec<PathBuf>>,
}

impl DllLoader {
    /// Create a new DLL loader
    pub fn new() -> Self {
        Self {
            libraries: RwLock::new(HashMap::new()),
            search_paths: RwLock::new(Vec::new()),
        }
    }

    /// Add DLL search path
    pub fn add_search_path(&self, path: impl AsRef<Path>) {
        self.search_paths.write().push(path.as_ref().to_path_buf());
    }

    /// Load DLL
    pub fn load(&self, name: &str, path: impl AsRef<Path>) -> FrameworkResult<()> {
        let path = path.as_ref();
        
        // Try direct loading
        let lib = unsafe { Library::new(path) }
            .map_err(|e| FrameworkError::InterfaceError(format!(
                "Failed to load DLL '{}': {}", path.display(), e
            )))?;

        self.libraries.write().insert(name.to_string(), lib);
        log::info!("DLL loaded: {} from {}", name, path.display());
        Ok(())
    }

    /// Load DLL from search paths
    pub fn load_from_search_paths(&self, dll_name: &str) -> FrameworkResult<()> {
        let search_paths = self.search_paths.read().clone();
        
        for search_path in search_paths {
            let full_path = search_path.join(dll_name);
            if full_path.exists() {
                return self.load(dll_name, &full_path);
            }
        }

        Err(FrameworkError::InterfaceError(format!(
            "DLL '{}' not found in search paths", dll_name
        )))
    }

    /// Unload DLL
    pub fn unload(&self, name: &str) -> FrameworkResult<()> {
        self.libraries.write().remove(name);
        log::info!("DLL unloaded: {}", name);
        Ok(())
    }

    /// Call function in DLL (returns string)
    pub fn call_string_fn(&self, dll_name: &str, fn_name: &str) -> FrameworkResult<String> {
        let libraries = self.libraries.read();
        let lib = libraries.get(dll_name)
            .ok_or_else(|| FrameworkError::InterfaceError(format!(
                "DLL '{}' not loaded", dll_name
            )))?;

        unsafe {
            let func: Symbol<'_, unsafe extern "C" fn() -> *mut c_char> = lib.get(fn_name.as_bytes())
                .map_err(|e| FrameworkError::InterfaceError(format!(
                    "Failed to get function '{}': {}", fn_name, e
                )))?;

            let result_ptr = func();
            if result_ptr.is_null() {
                return Err(FrameworkError::InterfaceError(
                    "Function returned null pointer".to_string()
                ));
            }

            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            
            // Try to call free function: try {prefix}_free_string
            // where prefix is extracted from function name (e.g., sysinfo_get_summary -> sysinfo)
            let fn_parts: Vec<&str> = fn_name.splitn(2, '_').collect();
            if fn_parts.len() >= 2 {
                let free_fn_name = format!("{}_free_string", fn_parts[0]);
                if let Ok(free_func) = lib.get::<unsafe extern "C" fn(*mut c_char)>(
                    free_fn_name.as_bytes()
                ) {
                    free_func(result_ptr);
                }
            }

            Ok(result)
        }
    }

    /// Call function in DLL (returns u32)
    pub fn call_u32_fn(&self, dll_name: &str, fn_name: &str) -> FrameworkResult<u32> {
        let libraries = self.libraries.read();
        let lib = libraries.get(dll_name)
            .ok_or_else(|| FrameworkError::InterfaceError(format!(
                "DLL '{}' not loaded", dll_name
            )))?;

        unsafe {
            let func: Symbol<'_, unsafe extern "C" fn() -> u32> = lib.get(fn_name.as_bytes())
                .map_err(|e| FrameworkError::InterfaceError(format!(
                    "Failed to get function '{}': {}", fn_name, e
                )))?;

            Ok(func())
        }
    }

    /// Call function in DLL (returns u64)
    pub fn call_u64_fn(&self, dll_name: &str, fn_name: &str) -> FrameworkResult<u64> {
        let libraries = self.libraries.read();
        let lib = libraries.get(dll_name)
            .ok_or_else(|| FrameworkError::InterfaceError(format!(
                "DLL '{}' not loaded", dll_name
            )))?;

        unsafe {
            let func: Symbol<'_, unsafe extern "C" fn() -> u64> = lib.get(fn_name.as_bytes())
                .map_err(|e| FrameworkError::InterfaceError(format!(
                    "Failed to get function '{}': {}", fn_name, e
                )))?;

            Ok(func())
        }
    }

    /// Call function in DLL (returns f64)
    pub fn call_f64_fn(&self, dll_name: &str, fn_name: &str) -> FrameworkResult<f64> {
        let libraries = self.libraries.read();
        let lib = libraries.get(dll_name)
            .ok_or_else(|| FrameworkError::InterfaceError(format!(
                "DLL '{}' not loaded", dll_name
            )))?;

        unsafe {
            let func: Symbol<'_, unsafe extern "C" fn() -> f64> = lib.get(fn_name.as_bytes())
                .map_err(|e| FrameworkError::InterfaceError(format!(
                    "Failed to get function '{}': {}", fn_name, e
                )))?;

            Ok(func())
        }
    }

    /// Check if DLL is loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        self.libraries.read().contains_key(name)
    }

    /// Get list of loaded DLLs
    pub fn list_loaded(&self) -> Vec<String> {
        self.libraries.read().keys().cloned().collect()
    }

    /// Get search paths
    pub fn get_search_paths(&self) -> Vec<PathBuf> {
        self.search_paths.read().clone()
    }
}

impl Default for DllLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dll_loader_creation() {
        let loader = DllLoader::new();
        assert!(loader.list_loaded().is_empty());
    }

    #[test]
    fn test_search_path() {
        let loader = DllLoader::new();
        loader.add_search_path("/tmp");
        // Search path has been added
    }
}
