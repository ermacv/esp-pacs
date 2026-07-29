#[doc = "Register `IMM_HP_CK_POWER_0` reader"]
pub type R = crate::R<IMM_HP_CK_POWER_0_SPEC>;
#[doc = "Register `IMM_HP_CK_POWER_0` writer"]
pub type W = crate::W<IMM_HP_CK_POWER_0_SPEC>;
#[doc = "Field `TIE_LOW_GLOBAL_XTALX2_ICG` reader - need_des"]
pub type TIE_LOW_GLOBAL_XTALX2_ICG_R = crate::BitReader;
#[doc = "Field `TIE_LOW_GLOBAL_XTALX2_ICG` writer - need_des"]
pub type TIE_LOW_GLOBAL_XTALX2_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_GLOBAL_XTAL_ICG` reader - need_des"]
pub type TIE_LOW_GLOBAL_XTAL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_LOW_GLOBAL_XTAL_ICG` writer - need_des"]
pub type TIE_LOW_GLOBAL_XTAL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_I2C_RETENTION` reader - need_des"]
pub type TIE_LOW_I2C_RETENTION_R = crate::BitReader;
#[doc = "Field `TIE_LOW_I2C_RETENTION` writer - need_des"]
pub type TIE_LOW_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_BB_I2C` reader - need_des"]
pub type TIE_LOW_XPD_BB_I2C_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_BB_I2C` writer - need_des"]
pub type TIE_LOW_XPD_BB_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_XTALX2` reader - need_des"]
pub type TIE_LOW_XPD_XTALX2_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_XTALX2` writer - need_des"]
pub type TIE_LOW_XPD_XTALX2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_XTAL` reader - need_des"]
pub type TIE_LOW_XPD_XTAL_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_XTAL` writer - need_des"]
pub type TIE_LOW_XPD_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_GLOBAL_XTALX2_ICG` reader - need_des"]
pub type TIE_HIGH_GLOBAL_XTALX2_ICG_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_GLOBAL_XTALX2_ICG` writer - need_des"]
pub type TIE_HIGH_GLOBAL_XTALX2_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_GLOBAL_XTAL_ICG` reader - need_des"]
pub type TIE_HIGH_GLOBAL_XTAL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_GLOBAL_XTAL_ICG` writer - need_des"]
pub type TIE_HIGH_GLOBAL_XTAL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_I2C_RETENTION` reader - need_des"]
pub type TIE_HIGH_I2C_RETENTION_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_I2C_RETENTION` writer - need_des"]
pub type TIE_HIGH_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_BB_I2C` reader - need_des"]
pub type TIE_HIGH_XPD_BB_I2C_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_BB_I2C` writer - need_des"]
pub type TIE_HIGH_XPD_BB_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_XTALX2` reader - need_des"]
pub type TIE_HIGH_XPD_XTALX2_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_XTALX2` writer - need_des"]
pub type TIE_HIGH_XPD_XTALX2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_XTAL` reader - need_des"]
pub type TIE_HIGH_XPD_XTAL_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_XTAL` writer - need_des"]
pub type TIE_HIGH_XPD_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn tie_low_global_xtalx2_icg(&self) -> TIE_LOW_GLOBAL_XTALX2_ICG_R {
        TIE_LOW_GLOBAL_XTALX2_ICG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn tie_low_global_xtal_icg(&self) -> TIE_LOW_GLOBAL_XTAL_ICG_R {
        TIE_LOW_GLOBAL_XTAL_ICG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn tie_low_i2c_retention(&self) -> TIE_LOW_I2C_RETENTION_R {
        TIE_LOW_I2C_RETENTION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_bb_i2c(&self) -> TIE_LOW_XPD_BB_I2C_R {
        TIE_LOW_XPD_BB_I2C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_xtalx2(&self) -> TIE_LOW_XPD_XTALX2_R {
        TIE_LOW_XPD_XTALX2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_xtal(&self) -> TIE_LOW_XPD_XTAL_R {
        TIE_LOW_XPD_XTAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn tie_high_global_xtalx2_icg(&self) -> TIE_HIGH_GLOBAL_XTALX2_ICG_R {
        TIE_HIGH_GLOBAL_XTALX2_ICG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn tie_high_global_xtal_icg(&self) -> TIE_HIGH_GLOBAL_XTAL_ICG_R {
        TIE_HIGH_GLOBAL_XTAL_ICG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn tie_high_i2c_retention(&self) -> TIE_HIGH_I2C_RETENTION_R {
        TIE_HIGH_I2C_RETENTION_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_bb_i2c(&self) -> TIE_HIGH_XPD_BB_I2C_R {
        TIE_HIGH_XPD_BB_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_xtalx2(&self) -> TIE_HIGH_XPD_XTALX2_R {
        TIE_HIGH_XPD_XTALX2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_xtal(&self) -> TIE_HIGH_XPD_XTAL_R {
        TIE_HIGH_XPD_XTAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMM_HP_CK_POWER_0")
            .field(
                "tie_low_global_xtalx2_icg",
                &self.tie_low_global_xtalx2_icg(),
            )
            .field("tie_low_global_xtal_icg", &self.tie_low_global_xtal_icg())
            .field("tie_low_i2c_retention", &self.tie_low_i2c_retention())
            .field("tie_low_xpd_bb_i2c", &self.tie_low_xpd_bb_i2c())
            .field("tie_low_xpd_xtalx2", &self.tie_low_xpd_xtalx2())
            .field("tie_low_xpd_xtal", &self.tie_low_xpd_xtal())
            .field(
                "tie_high_global_xtalx2_icg",
                &self.tie_high_global_xtalx2_icg(),
            )
            .field("tie_high_global_xtal_icg", &self.tie_high_global_xtal_icg())
            .field("tie_high_i2c_retention", &self.tie_high_i2c_retention())
            .field("tie_high_xpd_bb_i2c", &self.tie_high_xpd_bb_i2c())
            .field("tie_high_xpd_xtalx2", &self.tie_high_xpd_xtalx2())
            .field("tie_high_xpd_xtal", &self.tie_high_xpd_xtal())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn tie_low_global_xtalx2_icg(
        &mut self,
    ) -> TIE_LOW_GLOBAL_XTALX2_ICG_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_LOW_GLOBAL_XTALX2_ICG_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn tie_low_global_xtal_icg(
        &mut self,
    ) -> TIE_LOW_GLOBAL_XTAL_ICG_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_LOW_GLOBAL_XTAL_ICG_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn tie_low_i2c_retention(&mut self) -> TIE_LOW_I2C_RETENTION_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_LOW_I2C_RETENTION_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_bb_i2c(&mut self) -> TIE_LOW_XPD_BB_I2C_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_LOW_XPD_BB_I2C_W::new(self, 3)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_xtalx2(&mut self) -> TIE_LOW_XPD_XTALX2_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_LOW_XPD_XTALX2_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_xtal(&mut self) -> TIE_LOW_XPD_XTAL_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_LOW_XPD_XTAL_W::new(self, 6)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn tie_high_global_xtalx2_icg(
        &mut self,
    ) -> TIE_HIGH_GLOBAL_XTALX2_ICG_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_HIGH_GLOBAL_XTALX2_ICG_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn tie_high_global_xtal_icg(
        &mut self,
    ) -> TIE_HIGH_GLOBAL_XTAL_ICG_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_HIGH_GLOBAL_XTAL_ICG_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn tie_high_i2c_retention(
        &mut self,
    ) -> TIE_HIGH_I2C_RETENTION_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_HIGH_I2C_RETENTION_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_bb_i2c(&mut self) -> TIE_HIGH_XPD_BB_I2C_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_HIGH_XPD_BB_I2C_W::new(self, 28)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_xtalx2(&mut self) -> TIE_HIGH_XPD_XTALX2_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_HIGH_XPD_XTALX2_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_xtal(&mut self) -> TIE_HIGH_XPD_XTAL_W<'_, IMM_HP_CK_POWER_0_SPEC> {
        TIE_HIGH_XPD_XTAL_W::new(self, 31)
    }
}
#[doc = "SOURCE\\[S31_ESP_PACS_BASE_SVD,S31_PMU_HEADERS,BLOB_LIBPHY_PHY_OPEN_I2C_XPD_NEW\\]; CONFIDENCE\\[exact-s31-layout-and-instruction-use\\]. Immediate HP clock/power tie controls. The vendor header marks these fields WT, but the complete ESP32-S31 libphy.a\\[phy_reg.o\\] phy_open_i2c_xpd_new body loads this register before masking and storing it. Model it read-write so the PAC preserves the evidenced read/modify/write operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`imm_hp_ck_power_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_ck_power_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMM_HP_CK_POWER_0_SPEC;
impl crate::RegisterSpec for IMM_HP_CK_POWER_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imm_hp_ck_power_0::R`](R) reader structure"]
impl crate::Readable for IMM_HP_CK_POWER_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imm_hp_ck_power_0::W`](W) writer structure"]
impl crate::Writable for IMM_HP_CK_POWER_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_HP_CK_POWER_0 to value 0"]
impl crate::Resettable for IMM_HP_CK_POWER_0_SPEC {}
