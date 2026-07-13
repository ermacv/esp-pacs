#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x3c],
    hp_emac_ctrl: HP_EMAC_CTRL,
    hp_emac_ref_ctrl: HP_EMAC_REF_CTRL,
    hp_emac_rmii_pad_ctrl: HP_EMAC_RMII_PAD_CTRL,
    hp_emac_rmii_ctrl: HP_EMAC_RMII_CTRL,
    hp_emac_rx_ctrl: HP_EMAC_RX_CTRL,
    hp_emac_tx_ctrl: HP_EMAC_TX_CTRL,
    _reserved6: [u8; 0x0c],
    gmac_ctrl0: GMAC_CTRL0,
}
impl RegisterBlock {
    #[doc = "0x3c - EMAC reset control"]
    #[inline(always)]
    pub const fn hp_emac_ctrl(&self) -> &HP_EMAC_CTRL {
        &self.hp_emac_ctrl
    }
    #[doc = "0x40 - EMAC reference clock control"]
    #[inline(always)]
    pub const fn hp_emac_ref_ctrl(&self) -> &HP_EMAC_REF_CTRL {
        &self.hp_emac_ref_ctrl
    }
    #[doc = "0x44 - EMAC RMII pad clock control"]
    #[inline(always)]
    pub const fn hp_emac_rmii_pad_ctrl(&self) -> &HP_EMAC_RMII_PAD_CTRL {
        &self.hp_emac_rmii_pad_ctrl
    }
    #[doc = "0x48 - EMAC RMII clock control"]
    #[inline(always)]
    pub const fn hp_emac_rmii_ctrl(&self) -> &HP_EMAC_RMII_CTRL {
        &self.hp_emac_rmii_ctrl
    }
    #[doc = "0x4c - EMAC receive clock control"]
    #[inline(always)]
    pub const fn hp_emac_rx_ctrl(&self) -> &HP_EMAC_RX_CTRL {
        &self.hp_emac_rx_ctrl
    }
    #[doc = "0x50 - EMAC transmit clock control"]
    #[inline(always)]
    pub const fn hp_emac_tx_ctrl(&self) -> &HP_EMAC_TX_CTRL {
        &self.hp_emac_tx_ctrl
    }
    #[doc = "0x60 - GMAC interface and low-power control"]
    #[inline(always)]
    pub const fn gmac_ctrl0(&self) -> &GMAC_CTRL0 {
        &self.gmac_ctrl0
    }
}
#[doc = "HP_EMAC_CTRL (rw) register accessor: EMAC reset control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_emac_ctrl`] module"]
pub type HP_EMAC_CTRL = crate::Reg<hp_emac_ctrl::HP_EMAC_CTRL_SPEC>;
#[doc = "EMAC reset control"]
pub mod hp_emac_ctrl;
#[doc = "HP_EMAC_REF_CTRL (rw) register accessor: EMAC reference clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_ref_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_ref_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_emac_ref_ctrl`] module"]
pub type HP_EMAC_REF_CTRL = crate::Reg<hp_emac_ref_ctrl::HP_EMAC_REF_CTRL_SPEC>;
#[doc = "EMAC reference clock control"]
pub mod hp_emac_ref_ctrl;
#[doc = "HP_EMAC_RMII_PAD_CTRL (rw) register accessor: EMAC RMII pad clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_rmii_pad_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_rmii_pad_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_emac_rmii_pad_ctrl`] module"]
pub type HP_EMAC_RMII_PAD_CTRL = crate::Reg<hp_emac_rmii_pad_ctrl::HP_EMAC_RMII_PAD_CTRL_SPEC>;
#[doc = "EMAC RMII pad clock control"]
pub mod hp_emac_rmii_pad_ctrl;
#[doc = "HP_EMAC_RMII_CTRL (rw) register accessor: EMAC RMII clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_rmii_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_rmii_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_emac_rmii_ctrl`] module"]
pub type HP_EMAC_RMII_CTRL = crate::Reg<hp_emac_rmii_ctrl::HP_EMAC_RMII_CTRL_SPEC>;
#[doc = "EMAC RMII clock control"]
pub mod hp_emac_rmii_ctrl;
#[doc = "HP_EMAC_RX_CTRL (rw) register accessor: EMAC receive clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_emac_rx_ctrl`] module"]
pub type HP_EMAC_RX_CTRL = crate::Reg<hp_emac_rx_ctrl::HP_EMAC_RX_CTRL_SPEC>;
#[doc = "EMAC receive clock control"]
pub mod hp_emac_rx_ctrl;
#[doc = "HP_EMAC_TX_CTRL (rw) register accessor: EMAC transmit clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_emac_tx_ctrl`] module"]
pub type HP_EMAC_TX_CTRL = crate::Reg<hp_emac_tx_ctrl::HP_EMAC_TX_CTRL_SPEC>;
#[doc = "EMAC transmit clock control"]
pub mod hp_emac_tx_ctrl;
#[doc = "GMAC_CTRL0 (rw) register accessor: GMAC interface and low-power control\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_ctrl0`] module"]
pub type GMAC_CTRL0 = crate::Reg<gmac_ctrl0::GMAC_CTRL0_SPEC>;
#[doc = "GMAC interface and low-power control"]
pub mod gmac_ctrl0;
