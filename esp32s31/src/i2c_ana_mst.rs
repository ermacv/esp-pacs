#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c0_ctrl: I2C0_CTRL,
    i2c1_ctrl: I2C1_CTRL,
    i2c0_conf: I2C0_CONF,
    i2c1_conf: I2C1_CONF,
    i2c_burst_conf: I2C_BURST_CONF,
    i2c_burst_status: I2C_BURST_STATUS,
    ana_conf0: ANA_CONF0,
    ana_conf1: ANA_CONF1,
    ana_conf2: ANA_CONF2,
    i2c0_ctrl1: I2C0_CTRL1,
    i2c1_ctrl1: I2C1_CTRL1,
    hw_i2c_ctrl: HW_I2C_CTRL,
    i2c_mst_nouse: I2C_MST_NOUSE,
    ext_i2c_mask: EXT_I2C_MASK,
    i2c_mask: I2C_MASK,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog-register I2C host-0 command and completion word. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_master_reset, phy_chip_i2c_readReg_org and phy_chip_i2c_writeReg bodies."]
    #[inline(always)]
    pub const fn i2c0_ctrl(&self) -> &I2C0_CTRL {
        &self.i2c0_ctrl
    }
    #[doc = "0x04 - Analog-register I2C host-1 command and completion word. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_master_reset, phy_chip_i2c_readReg_org and phy_chip_i2c_writeReg bodies."]
    #[inline(always)]
    pub const fn i2c1_ctrl(&self) -> &I2C1_CTRL {
        &self.i2c1_ctrl
    }
    #[doc = "0x08 - I2C0_CONF"]
    #[inline(always)]
    pub const fn i2c0_conf(&self) -> &I2C0_CONF {
        &self.i2c0_conf
    }
    #[doc = "0x0c - I2C1_CONF"]
    #[inline(always)]
    pub const fn i2c1_conf(&self) -> &I2C1_CONF {
        &self.i2c1_conf
    }
    #[doc = "0x10 - I2C_BURST_CONF"]
    #[inline(always)]
    pub const fn i2c_burst_conf(&self) -> &I2C_BURST_CONF {
        &self.i2c_burst_conf
    }
    #[doc = "0x14 - I2C_BURST_STATUS"]
    #[inline(always)]
    pub const fn i2c_burst_status(&self) -> &I2C_BURST_STATUS {
        &self.i2c_burst_status
    }
    #[doc = "0x18 - Shared analog-I2C master control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2cmst_reg_init and phy_bbpll_cal bodies identify the PHY register-mode and BBPLL calibration fields."]
    #[inline(always)]
    pub const fn ana_conf0(&self) -> &ANA_CONF0 {
        &self.ana_conf0
    }
    #[doc = "0x1c - Shared analog-I2C routing/read-mask word. SOURCE\\[BLOB_LIBPHY_PHY_I2C\\]; phy_i2c_get_reg_mask publishes the complement of the selected 16-bit block mask. ESP32-S31 regi2c routing also consumes the low 24-bit configuration image."]
    #[inline(always)]
    pub const fn ana_conf1(&self) -> &ANA_CONF1 {
        &self.ana_conf1
    }
    #[doc = "0x20 - Shared analog-I2C host-selection word. SOURCE\\[BLOB_LIBPHY_PHY_I2C\\]; the S31 host callback replaces bits 17:4 with 0x3fa0. The internal subfield meanings remain unknown."]
    #[inline(always)]
    pub const fn ana_conf2(&self) -> &ANA_CONF2 {
        &self.ana_conf2
    }
    #[doc = "0x24 - Host-0 timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads."]
    #[inline(always)]
    pub const fn i2c0_ctrl1(&self) -> &I2C0_CTRL1 {
        &self.i2c0_ctrl1
    }
    #[doc = "0x28 - Host-1 timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads."]
    #[inline(always)]
    pub const fn i2c1_ctrl1(&self) -> &I2C1_CTRL1 {
        &self.i2c1_ctrl1
    }
    #[doc = "0x2c - Hardware-host timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads."]
    #[inline(always)]
    pub const fn hw_i2c_ctrl(&self) -> &HW_I2C_CTRL {
        &self.hw_i2c_ctrl
    }
    #[doc = "0x30 - I2C_MST_NOUSE"]
    #[inline(always)]
    pub const fn i2c_mst_nouse(&self) -> &I2C_MST_NOUSE {
        &self.i2c_mst_nouse
    }
    #[doc = "0x34 - EXT_I2C_MASK"]
    #[inline(always)]
    pub const fn ext_i2c_mask(&self) -> &EXT_I2C_MASK {
        &self.ext_i2c_mask
    }
    #[doc = "0x38 - I2C_MASK"]
    #[inline(always)]
    pub const fn i2c_mask(&self) -> &I2C_MASK {
        &self.i2c_mask
    }
    #[doc = "0x3c - DATE"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "I2C0_CTRL (rw) register accessor: Analog-register I2C host-0 command and completion word. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_master_reset, phy_chip_i2c_readReg_org and phy_chip_i2c_writeReg bodies.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctrl`] module"]
pub type I2C0_CTRL = crate::Reg<i2c0_ctrl::I2C0_CTRL_SPEC>;
#[doc = "Analog-register I2C host-0 command and completion word. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_master_reset, phy_chip_i2c_readReg_org and phy_chip_i2c_writeReg bodies."]
pub mod i2c0_ctrl;
#[doc = "I2C1_CTRL (rw) register accessor: Analog-register I2C host-1 command and completion word. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_master_reset, phy_chip_i2c_readReg_org and phy_chip_i2c_writeReg bodies.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctrl`] module"]
pub type I2C1_CTRL = crate::Reg<i2c1_ctrl::I2C1_CTRL_SPEC>;
#[doc = "Analog-register I2C host-1 command and completion word. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_master_reset, phy_chip_i2c_readReg_org and phy_chip_i2c_writeReg bodies."]
pub mod i2c1_ctrl;
#[doc = "I2C0_CONF (rw) register accessor: I2C0_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_conf`] module"]
pub type I2C0_CONF = crate::Reg<i2c0_conf::I2C0_CONF_SPEC>;
#[doc = "I2C0_CONF"]
pub mod i2c0_conf;
#[doc = "I2C1_CONF (rw) register accessor: I2C1_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_conf`] module"]
pub type I2C1_CONF = crate::Reg<i2c1_conf::I2C1_CONF_SPEC>;
#[doc = "I2C1_CONF"]
pub mod i2c1_conf;
#[doc = "I2C_BURST_CONF (rw) register accessor: I2C_BURST_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_burst_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_burst_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_burst_conf`] module"]
pub type I2C_BURST_CONF = crate::Reg<i2c_burst_conf::I2C_BURST_CONF_SPEC>;
#[doc = "I2C_BURST_CONF"]
pub mod i2c_burst_conf;
#[doc = "I2C_BURST_STATUS (rw) register accessor: I2C_BURST_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_burst_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_burst_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_burst_status`] module"]
pub type I2C_BURST_STATUS = crate::Reg<i2c_burst_status::I2C_BURST_STATUS_SPEC>;
#[doc = "I2C_BURST_STATUS"]
pub mod i2c_burst_status;
#[doc = "ANA_CONF0 (rw) register accessor: Shared analog-I2C master control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2cmst_reg_init and phy_bbpll_cal bodies identify the PHY register-mode and BBPLL calibration fields.\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf0`] module"]
pub type ANA_CONF0 = crate::Reg<ana_conf0::ANA_CONF0_SPEC>;
#[doc = "Shared analog-I2C master control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2cmst_reg_init and phy_bbpll_cal bodies identify the PHY register-mode and BBPLL calibration fields."]
pub mod ana_conf0;
#[doc = "ANA_CONF1 (rw) register accessor: Shared analog-I2C routing/read-mask word. SOURCE\\[BLOB_LIBPHY_PHY_I2C\\]; phy_i2c_get_reg_mask publishes the complement of the selected 16-bit block mask. ESP32-S31 regi2c routing also consumes the low 24-bit configuration image.\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf1`] module"]
pub type ANA_CONF1 = crate::Reg<ana_conf1::ANA_CONF1_SPEC>;
#[doc = "Shared analog-I2C routing/read-mask word. SOURCE\\[BLOB_LIBPHY_PHY_I2C\\]; phy_i2c_get_reg_mask publishes the complement of the selected 16-bit block mask. ESP32-S31 regi2c routing also consumes the low 24-bit configuration image."]
pub mod ana_conf1;
#[doc = "ANA_CONF2 (rw) register accessor: Shared analog-I2C host-selection word. SOURCE\\[BLOB_LIBPHY_PHY_I2C\\]; the S31 host callback replaces bits 17:4 with 0x3fa0. The internal subfield meanings remain unknown.\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_conf2`] module"]
pub type ANA_CONF2 = crate::Reg<ana_conf2::ANA_CONF2_SPEC>;
#[doc = "Shared analog-I2C host-selection word. SOURCE\\[BLOB_LIBPHY_PHY_I2C\\]; the S31 host callback replaces bits 17:4 with 0x3fa0. The internal subfield meanings remain unknown."]
pub mod ana_conf2;
#[doc = "I2C0_CTRL1 (rw) register accessor: Host-0 timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctrl1`] module"]
pub type I2C0_CTRL1 = crate::Reg<i2c0_ctrl1::I2C0_CTRL1_SPEC>;
#[doc = "Host-0 timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads."]
pub mod i2c0_ctrl1;
#[doc = "I2C1_CTRL1 (rw) register accessor: Host-1 timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctrl1`] module"]
pub type I2C1_CTRL1 = crate::Reg<i2c1_ctrl1::I2C1_CTRL1_SPEC>;
#[doc = "Host-1 timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads."]
pub mod i2c1_ctrl1;
#[doc = "HW_I2C_CTRL (rw) register accessor: Hardware-host timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads.\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_i2c_ctrl`] module"]
pub type HW_I2C_CTRL = crate::Reg<hw_i2c_ctrl::HW_I2C_CTRL_SPEC>;
#[doc = "Hardware-host timing/clock-selection control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_clk_sel updates the five-bit high field and then the six-bit low field using separate fresh reads."]
pub mod hw_i2c_ctrl;
#[doc = "I2C_MST_NOUSE (rw) register accessor: I2C_MST_NOUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_mst_nouse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_mst_nouse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_mst_nouse`] module"]
pub type I2C_MST_NOUSE = crate::Reg<i2c_mst_nouse::I2C_MST_NOUSE_SPEC>;
#[doc = "I2C_MST_NOUSE"]
pub mod i2c_mst_nouse;
#[doc = "EXT_I2C_MASK (rw) register accessor: EXT_I2C_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_i2c_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_i2c_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_i2c_mask`] module"]
pub type EXT_I2C_MASK = crate::Reg<ext_i2c_mask::EXT_I2C_MASK_SPEC>;
#[doc = "EXT_I2C_MASK"]
pub mod ext_i2c_mask;
#[doc = "I2C_MASK (rw) register accessor: I2C_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_mask`] module"]
pub type I2C_MASK = crate::Reg<i2c_mask::I2C_MASK_SPEC>;
#[doc = "I2C_MASK"]
pub mod i2c_mask;
#[doc = "DATE (rw) register accessor: DATE\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "DATE"]
pub mod date;
