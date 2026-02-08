use std::ops::BitOr;

use crate::c::macos::graphics::core_graphics::{CGPoint, CGRect, CGSize};

pub type NSRect = CGRect;

pub type NSPoint = CGPoint;

pub type NSSize = CGSize;

pub type NSUInteger = usize;

pub struct NSWindowStyleMask(NSUInteger);
impl BitOr for NSWindowStyleMask {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl NSWindowStyleMask {
    pub const NS_WINDOW_STYLE_MASK_BORDERLESS: Self = Self(0);
    pub const NS_WINDOW_STYLE_MASK_TITLED: Self = Self(1 << 0);
    pub const NS_WINDOW_STYLE_MASK_CLOSABLE: Self = Self(1 << 1);
    pub const NS_WINDOW_STYLE_MASK_MINIATURIZABLE: Self = Self(1 << 2);
    pub const NS_WINDOW_STYLE_MASK_RESIZABLE: Self = Self(1 << 3);
    pub const NS_WINDOW_STYLE_MASK_UTILITY_WINDOW: Self = Self(1 << 4);
    pub const NS_WINDOW_STYLE_MASK_DOC_MODAL_WINDOW: Self = Self(1 << 6);
    pub const NS_WINDOW_STYLE_MASK_NONACTIVATING_PANEL: Self = Self(1 << 7);
    pub const NS_WINDOW_STYLE_MASK_TEXTURED_BACKGROUND: Self = Self(1 << 8);
    pub const NS_WINDODW_STYLE_MASK_UNIFIED_TITLE_AND_TOOLBAR: Self = Self(1 << 12);
    pub const NS_WINDOW_STYLE_MASK_HUD_WINDOW: Self = Self(1 << 13);
    pub const NS_WINDOW_STYLE_MASK_FULL_SCREEN: Self = Self(1 << 14);
    pub const NS_WINDOW_STYLE_MASK_FULL_SIZE_CONTENT_VIEW: Self = Self(1 << 15);
}

pub struct NSBackingStoreType(usize);
impl BitOr for NSBackingStoreType {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl NSBackingStoreType {
    pub const NS_BACKING_STORE_BUFFERED: Self = Self(1 << 1);
}