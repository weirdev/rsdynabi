use crate::{
    manual_interop_imax::IMaxAdapter, manual_interop_imin::IMinAdapter,
    manual_interop_imin_and_max::IMinAndMaxAdapter, manual_shim_min_max_functs::*,
};

pub fn getMinnable() -> IMinAdapter {
    let ptr = unsafe { MinMaxFuncts_getMinnable() };
    IMinAdapter { ptr }
}

pub fn getMaxable() -> IMaxAdapter {
    let ptr = unsafe { MinMaxFuncts_getMaxable() };
    IMaxAdapter { ptr }
}

pub fn getMinMaxable() -> IMinAndMaxAdapter {
    let ptr = unsafe { MinMaxFuncts_getMinMaxable() };
    IMinAndMaxAdapter { ptr }
}
