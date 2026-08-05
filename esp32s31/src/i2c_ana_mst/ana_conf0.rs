#[doc = "Register `ANA_CONF0` reader"]
pub type R = crate::R<ANA_CONF0_SPEC>;
#[doc = "Register `ANA_CONF0` writer"]
pub type W = crate::W<ANA_CONF0_SPEC>;
#[doc = "Field `BBPLL_CAL_MODE_UNKNOWN` reader - BBPLL calibration selection. Complete phy_bbpll_cal uses only encoding 1 (disabled) and encoding 2 (enabled)."]
pub type BBPLL_CAL_MODE_UNKNOWN_R = crate::FieldReader;
#[doc = "Field `BBPLL_CAL_MODE_UNKNOWN` writer - BBPLL calibration selection. Complete phy_bbpll_cal uses only encoding 1 (disabled) and encoding 2 (enabled)."]
pub type BBPLL_CAL_MODE_UNKNOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `PHY_REGISTER_ENABLE` reader - Enable PHY analog-register master mode; set by complete phy_i2cmst_reg_init after selecting mode 2."]
pub type PHY_REGISTER_ENABLE_R = crate::BitReader;
#[doc = "Field `PHY_REGISTER_ENABLE` writer - Enable PHY analog-register master mode; set by complete phy_i2cmst_reg_init after selecting mode 2."]
pub type PHY_REGISTER_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_REGISTER_MODE` reader - PHY analog-register master mode; complete phy_i2cmst_reg_init selects encoding 2."]
pub type PHY_REGISTER_MODE_R = crate::FieldReader;
#[doc = "Field `PHY_REGISTER_MODE` writer - PHY analog-register master mode; complete phy_i2cmst_reg_init selects encoding 2."]
pub type PHY_REGISTER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `ANA_STATUS0` reader - "]
pub type ANA_STATUS0_R = crate::FieldReader;
#[doc = "Field `ANA_STATUS0` writer - "]
pub type ANA_STATUS0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:3 - BBPLL calibration selection. Complete phy_bbpll_cal uses only encoding 1 (disabled) and encoding 2 (enabled)."]
    #[inline(always)]
    pub fn bbpll_cal_mode_unknown(&self) -> BBPLL_CAL_MODE_UNKNOWN_R {
        BBPLL_CAL_MODE_UNKNOWN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Enable PHY analog-register master mode; set by complete phy_i2cmst_reg_init after selecting mode 2."]
    #[inline(always)]
    pub fn phy_register_enable(&self) -> PHY_REGISTER_ENABLE_R {
        PHY_REGISTER_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:10 - PHY analog-register master mode; complete phy_i2cmst_reg_init selects encoding 2."]
    #[inline(always)]
    pub fn phy_register_mode(&self) -> PHY_REGISTER_MODE_R {
        PHY_REGISTER_MODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ana_status0(&self) -> ANA_STATUS0_R {
        ANA_STATUS0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_CONF0")
            .field("bbpll_cal_mode_unknown", &self.bbpll_cal_mode_unknown())
            .field("phy_register_enable", &self.phy_register_enable())
            .field("phy_register_mode", &self.phy_register_mode())
            .field("ana_status0", &self.ana_status0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:3 - BBPLL calibration selection. Complete phy_bbpll_cal uses only encoding 1 (disabled) and encoding 2 (enabled)."]
    #[inline(always)]
    pub fn bbpll_cal_mode_unknown(&mut self) -> BBPLL_CAL_MODE_UNKNOWN_W<'_, ANA_CONF0_SPEC> {
        BBPLL_CAL_MODE_UNKNOWN_W::new(self, 2)
    }
    #[doc = "Bit 6 - Enable PHY analog-register master mode; set by complete phy_i2cmst_reg_init after selecting mode 2."]
    #[inline(always)]
    pub fn phy_register_enable(&mut self) -> PHY_REGISTER_ENABLE_W<'_, ANA_CONF0_SPEC> {
        PHY_REGISTER_ENABLE_W::new(self, 6)
    }
    #[doc = "Bits 9:10 - PHY analog-register master mode; complete phy_i2cmst_reg_init selects encoding 2."]
    #[inline(always)]
    pub fn phy_register_mode(&mut self) -> PHY_REGISTER_MODE_W<'_, ANA_CONF0_SPEC> {
        PHY_REGISTER_MODE_W::new(self, 9)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ana_status0(&mut self) -> ANA_STATUS0_W<'_, ANA_CONF0_SPEC> {
        ANA_STATUS0_W::new(self, 24)
    }
}
#[doc = "Shared analog-I2C master control. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2cmst_reg_init and phy_bbpll_cal bodies identify the PHY register-mode and BBPLL calibration fields.\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_CONF0_SPEC;
impl crate::RegisterSpec for ANA_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_conf0::R`](R) reader structure"]
impl crate::Readable for ANA_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_conf0::W`](W) writer structure"]
impl crate::Writable for ANA_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_CONF0 to value 0xe408"]
impl crate::Resettable for ANA_CONF0_SPEC {
    const RESET_VALUE: u32 = 0xe408;
}
