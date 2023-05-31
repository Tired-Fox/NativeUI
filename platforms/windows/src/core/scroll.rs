use std::mem::size_of;

use crate::core::constants::SB;
use native_core::Rect;
use style::{Dimensions, Overflow};
use windows::Win32::{
    Foundation::{HWND, WPARAM},
    UI::{
        Controls::{SetScrollInfo, ShowScrollBar},
        WindowsAndMessaging::{GetScrollInfo, ScrollWindow, SCROLLINFO, SIF_ALL, SIF_POS, SIF_PAGE, SIF_RANGE},
    },
};

use super::{loword, CharInfo};

pub fn get_scroll_info(handle: HWND, direction: SB::CONSTANTS) -> SCROLLINFO {
    let mut si = SCROLLINFO::default();
    si.cbSize = size_of::<SCROLLINFO>() as u32;
    si.fMask = SIF_ALL;
    unsafe {
        GetScrollInfo(handle, direction, &mut si as *mut SCROLLINFO);
    }
    si
}

pub fn update_scroll_info(handle: HWND, direction: SB::CONSTANTS, si: &mut SCROLLINFO) {
    unsafe {
        SetScrollInfo(handle, direction, si as *const SCROLLINFO, true);
        GetScrollInfo(handle, direction, si as *mut SCROLLINFO);
    }
}

pub fn vscroll(handle: HWND, wparam: WPARAM) {
    let ci = CharInfo::new(handle);
    let action = loword(wparam.0 as u32);

    let mut si = get_scroll_info(handle, SB::VERT);
    let y_pos = si.nPos.clone();

    match SB::COMMAND(action as i32) {
        SB::THUMBTRACK => {
            si.nPos = si.nTrackPos;
        }
        SB::LINEDOWN => {
            si.nPos += 1;
        }
        SB::LINEUP => {
            si.nPos -= 1;
        }
        SB::PAGEUP => si.nPos -= si.nPage as i32,
        SB::PAGEDOWN => si.nPos += si.nPage as i32,
        _ => (),
    }

    si.fMask = SIF_POS;
    update_scroll_info(handle, SB::VERT, &mut si);

    if si.nPos != y_pos {
        unsafe {
            ScrollWindow(handle, 0, ci.height * (y_pos - si.nPos), None, None);
        }
    }
}

pub fn hscroll(handle: HWND, wparam: WPARAM) {
    let ci = CharInfo::new(handle);
    let action = loword(wparam.0 as u32);

    let mut si = get_scroll_info(handle, SB::HORZ);
    unsafe {
        GetScrollInfo(handle, SB::HORZ, &mut si as *mut SCROLLINFO);
    }
    let x_pos = si.nPos.clone();
    match SB::COMMAND(action as i32) {
        SB::THUMBTRACK => {
            si.nPos = si.nTrackPos;
        }
        SB::LINERIGHT => {
            si.nPos += 1;
        }
        SB::LINELEFT => {
            si.nPos -= 1;
        }
        SB::PAGELEFT => si.nPos -= si.nPage as i32,
        SB::PAGERIGHT => si.nPos += si.nPage as i32,
        _ => (),
    }

    si.fMask = SIF_POS;
    update_scroll_info(handle, SB::HORZ, &mut si);

    if si.nPos != x_pos {
        unsafe {
            ScrollWindow(handle, ci.width * (x_pos - si.nPos), 0, None, None);
        }
    }
}

pub fn resize_scrollbars(handle: HWND, rect: &Rect, dimensions: Dimensions, point: (i32, i32)) {
    let ci = CharInfo::new(handle);
    let padding = dimensions
        .padding
        .calc(rect.width(), rect.height());

    let point = (point.0 + padding.3, point.1 + padding.2);

    // Show or hide scrollbar based on scrollbar state
    if dimensions.overflow_x == Overflow::Auto {
        if point.0 > rect.right {
            let mut si = get_scroll_info(handle, SB::HORZ);

            si.fMask = SIF_RANGE | SIF_PAGE;
            si.nMin = 0;
            si.nMax = point.0 / ci.width;
            si.nPage = (rect.width() / ci.width) as u32;
            unsafe { SetScrollInfo(handle, SB::HORZ, &si, true) };

            unsafe {
                ShowScrollBar(handle, SB::HORZ, true);
            }
        } else {
            unsafe {
                ShowScrollBar(handle, SB::HORZ, false);
            }
        }
    }

    if dimensions.overflow_y == Overflow::Auto {
        if point.1 > rect.bottom {
            let mut si = get_scroll_info(handle, SB::VERT);

            si.fMask = SIF_RANGE | SIF_PAGE;
            si.nMin = 0;
            si.nMax = point.1 / ci.height;
            si.nPage = (rect.height() / ci.height) as u32;
            unsafe { SetScrollInfo(handle, SB::VERT, &si, true) };

            unsafe {
                ShowScrollBar(handle, SB::VERT, true);
            }
        } else {
            unsafe {
                ShowScrollBar(handle, SB::VERT, false);
            }
        }
    }
}
