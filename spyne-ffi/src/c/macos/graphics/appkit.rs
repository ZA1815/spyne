use std::ops::BitOr;

use crate::c::macos::graphics::core_graphics::{CGPoint, CGRect, CGSize};

pub type NSRect = CGRect;

pub type NSPoint = CGPoint;

pub type NSSize = CGSize;

pub type NSUInteger = usize;

#[repr(transparent)]
pub struct NSWindowStyleMask(NSUInteger);
impl BitOr for NSWindowStyleMask {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
pub const NS_WINDOW_STYLE_MASK_BORDERLESS: NSWindowStyleMask = NSWindowStyleMask(0);
pub const NS_WINDOW_STYLE_MASK_TITLED: NSWindowStyleMask = NSWindowStyleMask(1 << 0);
pub const NS_WINDOW_STYLE_MASK_CLOSABLE: NSWindowStyleMask = NSWindowStyleMask(1 << 1);
pub const NS_WINDOW_STYLE_MASK_MINIATURIZABLE: NSWindowStyleMask = NSWindowStyleMask(1 << 2);
pub const NS_WINDOW_STYLE_MASK_RESIZABLE: NSWindowStyleMask = NSWindowStyleMask(1 << 3);
pub const NS_WINDOW_STYLE_MASK_UTILITY_WINDOW: NSWindowStyleMask = NSWindowStyleMask(1 << 4);
pub const NS_WINDOW_STYLE_MASK_DOC_MODAL_WINDOW: NSWindowStyleMask = NSWindowStyleMask(1 << 6);
pub const NS_WINDOW_STYLE_MASK_NONACTIVATING_PANEL: NSWindowStyleMask = NSWindowStyleMask(1 << 7);
pub const NS_WINDOW_STYLE_MASK_TEXTURED_BACKGROUND: NSWindowStyleMask = NSWindowStyleMask(1 << 8);
pub const NS_WINDOW_STYLE_MASK_UNIFIED_TITLE_AND_TOOLBAR: NSWindowStyleMask = NSWindowStyleMask(1 << 12);
pub const NS_WINDOW_STYLE_MASK_HUD_WINDOW: NSWindowStyleMask = NSWindowStyleMask(1 << 13);
pub const NS_WINDOW_STYLE_MASK_FULL_SCREEN: NSWindowStyleMask = NSWindowStyleMask(1 << 14);
pub const NS_WINDOW_STYLE_MASK_FULL_SIZE_CONTENT_VIEW: NSWindowStyleMask = NSWindowStyleMask(1 << 15);

#[repr(transparent)]
pub struct NSBackingStoreType(NSUInteger);
impl BitOr for NSBackingStoreType {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
pub const NS_BACKING_STORE_BUFFERED: NSBackingStoreType = NSBackingStoreType(1 << 1);