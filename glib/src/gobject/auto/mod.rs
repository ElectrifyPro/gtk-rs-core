// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod binding;
pub use self::binding::Binding;

#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
mod binding_group;
#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
pub use self::binding_group::BindingGroup;

#[cfg(feature = "v2_74")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_74")))]
mod signal_group;
#[cfg(feature = "v2_74")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_74")))]
pub use self::signal_group::SignalGroup;

mod flags;
pub use self::flags::BindingFlags;
pub use self::flags::SignalFlags;
pub use self::flags::TypeFlags;
