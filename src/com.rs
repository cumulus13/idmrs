// File        : idmrs/src/com.rs
// Author      : Hadi Cahyadi <cumulus13@gmail.com>
// Description : Late-bound COM automation against IDM's CIDMLinkTransmitter
//               (CLSID {ECF21EAB-3AA8-4355-82BE-F777990001DD}), calling
//               ICIDMLinkTransmitter2::SendLinkToIDM2, mirroring the Python
//               comtypes implementation in idm.py.
#![cfg(windows)]

use anyhow::{anyhow, Result};
use windows::core::{w, GUID, VARIANT};
use windows::Win32::Globalization::LOCALE_USER_DEFAULT;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, IDispatch, CLSCTX_LOCAL_SERVER,
    COINIT_APARTMENTTHREADED, DISPATCH_METHOD, DISPPARAMS,
};
use windows::Win32::UI::WindowsAndMessaging::{
    BringWindowToTop, EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
    SetForegroundWindow, ShowWindow, SW_SHOW,
};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};

const CLSID_IDMAN: GUID = GUID::from_values(
    0xECF21EAB,
    0x3AA8,
    0x4355,
    [0x82, 0xBE, 0xF7, 0x77, 0x99, 0x00, 0x01, 0xDD],
);

#[derive(Debug, thiserror::Error)]
pub enum IdmError {
    #[error("It seem IDM (Internet Download Manager) not installed, please install first !")]
    NotFound,
}

/// Build a VARIANT holding a BSTR from an optional string, or VT_EMPTY when None.
fn variant_str(v: Option<&str>) -> VARIANT {
    match v {
        Some(s) => VARIANT::from(s),
        None => VARIANT::default(),
    }
}

/// Parameters for SendLinkToIDM2, mirroring IDMan.download() in idm.py.
pub struct SendLinkArgs<'a> {
    pub link: &'a str,
    pub referrer: Option<&'a str>,
    pub cookie: Option<&'a str>,
    pub post_data: Option<&'a str>,
    pub user: Option<&'a str>,
    pub password: Option<&'a str>,
    pub path_to_save: Option<&'a str>,
    pub output: Option<&'a str>,
    pub lflag: i32,
    pub user_agent: Option<&'a str>,
}

/// Sends a link to IDM via late-bound COM automation.
pub fn send_link_to_idm(args: &SendLinkArgs) -> Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let disp: IDispatch = CoCreateInstance(&CLSID_IDMAN, None, CLSCTX_LOCAL_SERVER)
            .map_err(|_| anyhow!(IdmError::NotFound))?;

        let name = w!("SendLinkToIDM2");
        let mut dispid: i32 = 0;
        let names = [name];
        disp.GetIDsOfNames(
            &GUID::zeroed(),
            names.as_ptr(),
            1,
            LOCALE_USER_DEFAULT,
            &mut dispid,
        )?;

        let reserved1 = variant_str(args.user_agent);
        let reserved2 = VARIANT::default();

        // COM Invoke expects arguments in reverse order.
        let mut com_args: Vec<VARIANT> = vec![
            reserved2,
            reserved1,
            VARIANT::from(args.lflag),
            variant_str(args.output),
            variant_str(args.path_to_save),
            variant_str(args.password),
            variant_str(args.user),
            variant_str(args.post_data),
            variant_str(args.cookie),
            variant_str(args.referrer),
            VARIANT::from(args.link),
        ];

        let dp = DISPPARAMS {
            rgvarg: com_args.as_mut_ptr(),
            rgdispidNamedArgs: std::ptr::null_mut(),
            cArgs: com_args.len() as u32,
            cNamedArgs: 0,
        };

        let mut result = VARIANT::default();
        disp.Invoke(
            dispid,
            &GUID::zeroed(),
            LOCALE_USER_DEFAULT,
            DISPATCH_METHOD,
            &dp,
            Some(&mut result),
            None,
            None,
        )?;

        CoUninitialize();
    }
    Ok(())
}

/// Equivalent of IDMan.bring_to_top(): find the IDM main window by title
/// substring and bring it to the foreground.
pub fn bring_to_top() {
    unsafe {
        static mut FOUND: Option<HWND> = None;
        FOUND = None;

        unsafe extern "system" fn enum_proc(hwnd: HWND, _lparam: LPARAM) -> BOOL {
            if IsWindowVisible(hwnd).as_bool() {
                let len = GetWindowTextLengthW(hwnd);
                if len > 0 {
                    let mut buf = vec![0u16; (len + 1) as usize];
                    let read = GetWindowTextW(hwnd, &mut buf);
                    if read > 0 {
                        let title = String::from_utf16_lossy(&buf[..read as usize]);
                        if title.to_lowercase().contains("internet download manager") {
                            FOUND = Some(hwnd);
                            return BOOL(0); // stop enumeration
                        }
                    }
                }
            }
            BOOL(1)
        }

        let _ = EnumWindows(Some(enum_proc), LPARAM(0));

        if let Some(hwnd) = FOUND {
            let _ = ShowWindow(hwnd, SW_SHOW);
            let _ = SetForegroundWindow(hwnd);
            let _ = BringWindowToTop(hwnd);
        }
    }
}
