use crate::config::{
    BaseConfigOption, BorrowedOption, ConfigOption, ConfigSection,
    HiddenBorrowedOption, HidenConfigOptionT,
};
use crate::Weechat;
use std::borrow::Cow;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Deref;
use weechat_sys::{t_config_option, t_weechat_plugin};

/// Represents the settings for a new string config option.
#[derive(Default)]
pub struct StringOptionSettings {
    pub(crate) name: String,

    pub(crate) description: String,

    pub(crate) default_value: String,

    pub(crate) change_cb: Option<Box<dyn FnMut(&Weechat, &StringOpt)>>,

    pub(crate) check_cb: Option<Box<dyn FnMut(&Weechat, &StringOpt, Cow<str>)>>,
}

impl StringOptionSettings {
    pub fn new<N: Into<String>>(name: N) -> Self {
        StringOptionSettings {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn description<D: Into<String>>(mut self, descritpion: D) -> Self {
        self.description = descritpion.into();
        self
    }

    pub fn default_value<V: Into<String>>(mut self, value: V) -> Self {
        self.default_value = value.into();
        self
    }

    pub fn set_change_callback(
        mut self,
        callback: impl FnMut(&Weechat, &StringOpt) + 'static,
    ) -> Self {
        self.change_cb = Some(Box::new(callback));
        self
    }

    pub fn set_check_callback(
        mut self,
        callback: impl FnMut(&Weechat, &StringOpt, Cow<str>) + 'static,
    ) -> Self {
        self.check_cb = Some(Box::new(callback));
        self
    }
}

/// A config option with a boolean value.
pub struct StringOption<'a> {
    pub(crate) inner: StringOpt,
    pub(crate) section: PhantomData<&'a ConfigSection>,
}

pub struct StringOpt {
    pub(crate) ptr: *mut t_config_option,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

impl HiddenBorrowedOption for StringOpt {
    fn from_ptrs(
        option_ptr: *mut t_config_option,
        weechat_ptr: *mut t_weechat_plugin,
    ) -> Self {
        StringOpt {
            ptr: option_ptr,
            weechat_ptr,
        }
    }
}

impl BorrowedOption for StringOpt {}

impl<'a> Deref for StringOption<'a> {
    type Target = StringOpt;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl HidenConfigOptionT for StringOpt {
    fn get_ptr(&self) -> *mut t_config_option {
        self.ptr
    }

    fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat_ptr)
    }
}

impl<'a> HidenConfigOptionT for StringOption<'a> {
    fn get_ptr(&self) -> *mut t_config_option {
        self.ptr
    }

    fn get_weechat(&self) -> Weechat {
        Weechat::from_ptr(self.weechat_ptr)
    }
}

impl<'a> BaseConfigOption for StringOption<'a> {}
impl BaseConfigOption for StringOpt {}

impl<'a> ConfigOption<'a> for StringOpt {
    type R = Cow<'a, str>;

    fn value(&self) -> Self::R {
        let weechat = self.get_weechat();
        let config_string = weechat.get().config_string.unwrap();
        unsafe {
            let string = config_string(self.get_ptr());
            CStr::from_ptr(string).to_string_lossy()
        }
    }
}