#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    fc_00: FC_00,
    fc_01: FC_01,
    fc_02: FC_02,
    fc_03: FC_03,
    fc_04: FC_04,
    fc_05: FC_05,
    fc_06: FC_06,
    fc_07: FC_07,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock gating and termination resistor control"]
    #[inline(always)]
    pub const fn fc_00(&self) -> &FC_00 {
        &self.fc_00
    }
    #[doc = "0x04 - Squelch threshold and high-speed output swing"]
    #[inline(always)]
    pub const fn fc_01(&self) -> &FC_01 {
        &self.fc_01
    }
    #[doc = "0x08 - Bias current and output eye shape"]
    #[inline(always)]
    pub const fn fc_02(&self) -> &FC_02 {
        &self.fc_02
    }
    #[doc = "0x0c - PLL and TX clock phase"]
    #[inline(always)]
    pub const fn fc_03(&self) -> &FC_03 {
        &self.fc_03
    }
    #[doc = "0x10 - Test select (output left unconnected)"]
    #[inline(always)]
    pub const fn fc_04(&self) -> &FC_04 {
        &self.fc_04
    }
    #[doc = "0x14 - Receive and clock selection"]
    #[inline(always)]
    pub const fn fc_05(&self) -> &FC_05 {
        &self.fc_05
    }
    #[doc = "0x18 - Low-speed mode and high-speed turn-around"]
    #[inline(always)]
    pub const fn fc_06(&self) -> &FC_06 {
        &self.fc_06
    }
    #[doc = "0x1c - 3 ms counter and CLK_480 output timing"]
    #[inline(always)]
    pub const fn fc_07(&self) -> &FC_07 {
        &self.fc_07
    }
}
#[doc = "FC_00 (rw) register accessor: Clock gating and termination resistor control\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_00`] module"]
pub type FC_00 = crate::Reg<fc_00::FC_00_SPEC>;
#[doc = "Clock gating and termination resistor control"]
pub mod fc_00;
#[doc = "FC_01 (rw) register accessor: Squelch threshold and high-speed output swing\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_01`] module"]
pub type FC_01 = crate::Reg<fc_01::FC_01_SPEC>;
#[doc = "Squelch threshold and high-speed output swing"]
pub mod fc_01;
#[doc = "FC_02 (rw) register accessor: Bias current and output eye shape\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_02`] module"]
pub type FC_02 = crate::Reg<fc_02::FC_02_SPEC>;
#[doc = "Bias current and output eye shape"]
pub mod fc_02;
#[doc = "FC_03 (rw) register accessor: PLL and TX clock phase\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_03`] module"]
pub type FC_03 = crate::Reg<fc_03::FC_03_SPEC>;
#[doc = "PLL and TX clock phase"]
pub mod fc_03;
#[doc = "FC_04 (rw) register accessor: Test select (output left unconnected)\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_04`] module"]
pub type FC_04 = crate::Reg<fc_04::FC_04_SPEC>;
#[doc = "Test select (output left unconnected)"]
pub mod fc_04;
#[doc = "FC_05 (rw) register accessor: Receive and clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_05`] module"]
pub type FC_05 = crate::Reg<fc_05::FC_05_SPEC>;
#[doc = "Receive and clock selection"]
pub mod fc_05;
#[doc = "FC_06 (rw) register accessor: Low-speed mode and high-speed turn-around\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_06`] module"]
pub type FC_06 = crate::Reg<fc_06::FC_06_SPEC>;
#[doc = "Low-speed mode and high-speed turn-around"]
pub mod fc_06;
#[doc = "FC_07 (rw) register accessor: 3 ms counter and CLK_480 output timing\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_07`] module"]
pub type FC_07 = crate::Reg<fc_07::FC_07_SPEC>;
#[doc = "3 ms counter and CLK_480 output timing"]
pub mod fc_07;
