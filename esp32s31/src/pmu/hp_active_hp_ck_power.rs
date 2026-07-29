#[doc = "Register `HP_ACTIVE_HP_CK_POWER` reader"]
pub type R = crate::R<HP_ACTIVE_HP_CK_POWER_SPEC>;
#[doc = "Register `HP_ACTIVE_HP_CK_POWER` writer"]
pub type W = crate::W<HP_ACTIVE_HP_CK_POWER_SPEC>;
#[doc = "Field `ROM_OPEN_FE_BB_UNKNOWN_LOW` reader - SOURCE\\[ROM_REV0_PHY_OPEN_FE_BB_CLK\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. The complete ESP32-S31 rev0 ROM phy_open_fe_bb_clk body sets bits 3:0; their individual meanings remain unknown."]
pub type ROM_OPEN_FE_BB_UNKNOWN_LOW_R = crate::FieldReader;
#[doc = "Field `ROM_OPEN_FE_BB_UNKNOWN_LOW` writer - SOURCE\\[ROM_REV0_PHY_OPEN_FE_BB_CLK\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. The complete ESP32-S31 rev0 ROM phy_open_fe_bb_clk body sets bits 3:0; their individual meanings remain unknown."]
pub type ROM_OPEN_FE_BB_UNKNOWN_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_ACTIVE_XPD_XTALX2` reader - need_des"]
pub type HP_ACTIVE_XPD_XTALX2_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_XTALX2` writer - need_des"]
pub type HP_ACTIVE_XPD_XTALX2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_I2C_ISO_EN` reader - need_des"]
pub type HP_ACTIVE_I2C_ISO_EN_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_I2C_ISO_EN` writer - need_des"]
pub type HP_ACTIVE_I2C_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_I2C_RETENTION` reader - need_des"]
pub type HP_ACTIVE_I2C_RETENTION_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_I2C_RETENTION` writer - need_des"]
pub type HP_ACTIVE_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_BB_I2C` reader - need_des"]
pub type HP_ACTIVE_XPD_BB_I2C_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_BB_I2C` writer - need_des"]
pub type HP_ACTIVE_XPD_BB_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_PLL_I2C` reader - need_des"]
pub type HP_ACTIVE_XPD_PLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_PLL_I2C` writer - need_des"]
pub type HP_ACTIVE_XPD_PLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_BBPLL_I2C` reader - "]
pub type HP_ACTIVE_XPD_BBPLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_BBPLL_I2C` writer - "]
pub type HP_ACTIVE_XPD_BBPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_APLL_I2C` reader - "]
pub type HP_ACTIVE_XPD_APLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_APLL_I2C` writer - "]
pub type HP_ACTIVE_XPD_APLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_MPLL_I2C` reader - "]
pub type HP_ACTIVE_XPD_MPLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_MPLL_I2C` writer - "]
pub type HP_ACTIVE_XPD_MPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_PLL` reader - need_des"]
pub type HP_ACTIVE_XPD_PLL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_PLL` writer - need_des"]
pub type HP_ACTIVE_XPD_PLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_BBPLL` reader - "]
pub type HP_ACTIVE_XPD_BBPLL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_BBPLL` writer - "]
pub type HP_ACTIVE_XPD_BBPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_APLL` reader - "]
pub type HP_ACTIVE_XPD_APLL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_APLL` writer - "]
pub type HP_ACTIVE_XPD_APLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_MPLL` reader - "]
pub type HP_ACTIVE_XPD_MPLL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_MPLL` writer - "]
pub type HP_ACTIVE_XPD_MPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - SOURCE\\[ROM_REV0_PHY_OPEN_FE_BB_CLK\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. The complete ESP32-S31 rev0 ROM phy_open_fe_bb_clk body sets bits 3:0; their individual meanings remain unknown."]
    #[inline(always)]
    pub fn rom_open_fe_bb_unknown_low(&self) -> ROM_OPEN_FE_BB_UNKNOWN_LOW_R {
        ROM_OPEN_FE_BB_UNKNOWN_LOW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_xtalx2(&self) -> HP_ACTIVE_XPD_XTALX2_R {
        HP_ACTIVE_XPD_XTALX2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_iso_en(&self) -> HP_ACTIVE_I2C_ISO_EN_R {
        HP_ACTIVE_I2C_ISO_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_retention(&self) -> HP_ACTIVE_I2C_RETENTION_R {
        HP_ACTIVE_I2C_RETENTION_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_bb_i2c(&self) -> HP_ACTIVE_XPD_BB_I2C_R {
        HP_ACTIVE_XPD_BB_I2C_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_pll_i2c(&self) -> HP_ACTIVE_XPD_PLL_I2C_R {
        HP_ACTIVE_XPD_PLL_I2C_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn hp_active_xpd_bbpll_i2c(&self) -> HP_ACTIVE_XPD_BBPLL_I2C_R {
        HP_ACTIVE_XPD_BBPLL_I2C_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn hp_active_xpd_apll_i2c(&self) -> HP_ACTIVE_XPD_APLL_I2C_R {
        HP_ACTIVE_XPD_APLL_I2C_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn hp_active_xpd_mpll_i2c(&self) -> HP_ACTIVE_XPD_MPLL_I2C_R {
        HP_ACTIVE_XPD_MPLL_I2C_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_pll(&self) -> HP_ACTIVE_XPD_PLL_R {
        HP_ACTIVE_XPD_PLL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn hp_active_xpd_bbpll(&self) -> HP_ACTIVE_XPD_BBPLL_R {
        HP_ACTIVE_XPD_BBPLL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn hp_active_xpd_apll(&self) -> HP_ACTIVE_XPD_APLL_R {
        HP_ACTIVE_XPD_APLL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn hp_active_xpd_mpll(&self) -> HP_ACTIVE_XPD_MPLL_R {
        HP_ACTIVE_XPD_MPLL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_HP_CK_POWER")
            .field("hp_active_xpd_xtalx2", &self.hp_active_xpd_xtalx2())
            .field("hp_active_i2c_iso_en", &self.hp_active_i2c_iso_en())
            .field("hp_active_i2c_retention", &self.hp_active_i2c_retention())
            .field("hp_active_xpd_bb_i2c", &self.hp_active_xpd_bb_i2c())
            .field("hp_active_xpd_pll_i2c", &self.hp_active_xpd_pll_i2c())
            .field("hp_active_xpd_pll", &self.hp_active_xpd_pll())
            .field(
                "rom_open_fe_bb_unknown_low",
                &self.rom_open_fe_bb_unknown_low(),
            )
            .field("hp_active_xpd_bbpll_i2c", &self.hp_active_xpd_bbpll_i2c())
            .field("hp_active_xpd_apll_i2c", &self.hp_active_xpd_apll_i2c())
            .field("hp_active_xpd_mpll_i2c", &self.hp_active_xpd_mpll_i2c())
            .field("hp_active_xpd_bbpll", &self.hp_active_xpd_bbpll())
            .field("hp_active_xpd_apll", &self.hp_active_xpd_apll())
            .field("hp_active_xpd_mpll", &self.hp_active_xpd_mpll())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - SOURCE\\[ROM_REV0_PHY_OPEN_FE_BB_CLK\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. The complete ESP32-S31 rev0 ROM phy_open_fe_bb_clk body sets bits 3:0; their individual meanings remain unknown."]
    #[inline(always)]
    pub fn rom_open_fe_bb_unknown_low(
        &mut self,
    ) -> ROM_OPEN_FE_BB_UNKNOWN_LOW_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        ROM_OPEN_FE_BB_UNKNOWN_LOW_W::new(self, 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_xtalx2(
        &mut self,
    ) -> HP_ACTIVE_XPD_XTALX2_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_XTALX2_W::new(self, 19)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_iso_en(
        &mut self,
    ) -> HP_ACTIVE_I2C_ISO_EN_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_I2C_ISO_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_retention(
        &mut self,
    ) -> HP_ACTIVE_I2C_RETENTION_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_I2C_RETENTION_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_bb_i2c(
        &mut self,
    ) -> HP_ACTIVE_XPD_BB_I2C_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_BB_I2C_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_pll_i2c(
        &mut self,
    ) -> HP_ACTIVE_XPD_PLL_I2C_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_PLL_I2C_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn hp_active_xpd_bbpll_i2c(
        &mut self,
    ) -> HP_ACTIVE_XPD_BBPLL_I2C_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_BBPLL_I2C_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn hp_active_xpd_apll_i2c(
        &mut self,
    ) -> HP_ACTIVE_XPD_APLL_I2C_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_APLL_I2C_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn hp_active_xpd_mpll_i2c(
        &mut self,
    ) -> HP_ACTIVE_XPD_MPLL_I2C_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_MPLL_I2C_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_pll(&mut self) -> HP_ACTIVE_XPD_PLL_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_PLL_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn hp_active_xpd_bbpll(&mut self) -> HP_ACTIVE_XPD_BBPLL_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_BBPLL_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn hp_active_xpd_apll(&mut self) -> HP_ACTIVE_XPD_APLL_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_APLL_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn hp_active_xpd_mpll(&mut self) -> HP_ACTIVE_XPD_MPLL_W<'_, HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_MPLL_W::new(self, 30)
    }
}
#[doc = "SOURCE\\[S31_ESP_PACS_BASE_SVD,S31_PMU_HEADERS,ROM_REV0_PHY_OPEN_FE_BB_CLK\\]; CONFIDENCE\\[mixed-per-field\\]. HP-active clock/power controls. The complete ESP32-S31 rev0 ROM phy_open_fe_bb_clk body sets bits 3:0 and bit 22; the individual meanings of bits 3:0 remain unknown.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_ck_power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_ck_power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ACTIVE_HP_CK_POWER_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_HP_CK_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_hp_ck_power::R`](R) reader structure"]
impl crate::Readable for HP_ACTIVE_HP_CK_POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_active_hp_ck_power::W`](W) writer structure"]
impl crate::Writable for HP_ACTIVE_HP_CK_POWER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ACTIVE_HP_CK_POWER to value 0"]
impl crate::Resettable for HP_ACTIVE_HP_CK_POWER_SPEC {}
