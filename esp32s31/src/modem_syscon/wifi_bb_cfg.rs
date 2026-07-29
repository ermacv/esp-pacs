#[doc = "Register `WIFI_BB_CFG` reader"]
pub type R = crate::R<WIFI_BB_CFG_SPEC>;
#[doc = "Register `WIFI_BB_CFG` writer"]
pub type W = crate::W<WIFI_BB_CFG_SPEC>;
#[doc = "Field `COLD_START_CLEAR_UNKNOWN` reader - SOURCE\\[BLOB_LIBPHY_REGISTER_CHIPV7_PHY\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. register_chipv7_phy clears bit 0 together with WIFI_ENABLE before taking ownership of the powered PHY."]
pub type COLD_START_CLEAR_UNKNOWN_R = crate::BitReader;
#[doc = "Field `COLD_START_CLEAR_UNKNOWN` writer - SOURCE\\[BLOB_LIBPHY_REGISTER_CHIPV7_PHY\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. register_chipv7_phy clears bit 0 together with WIFI_ENABLE before taking ownership of the powered PHY."]
pub type COLD_START_CLEAR_UNKNOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_ENABLE` reader - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL,ROM_REV0_PHY_PBUS, BLOB_LIBPHY_REGISTER_CHIPV7_PHY\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol-and-consumers\\]. Complete phy_wifi_enable_set controls bit 1; phy_pbus_force_mode samples it and register_chipv7_phy initially clears it."]
pub type WIFI_ENABLE_R = crate::BitReader;
#[doc = "Field `WIFI_ENABLE` writer - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL,ROM_REV0_PHY_PBUS, BLOB_LIBPHY_REGISTER_CHIPV7_PHY\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol-and-consumers\\]. Complete phy_wifi_enable_set controls bit 1; phy_pbus_force_mode samples it and register_chipv7_phy initially clears it."]
pub type WIFI_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS_CBW_40_DIGITAL_UNKNOWN` reader - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol\\]. Complete phy_bb_bss_cbw40_dig replaces bits 3:2 with zero or encoding one."]
pub type BSS_CBW_40_DIGITAL_UNKNOWN_R = crate::FieldReader;
#[doc = "Field `BSS_CBW_40_DIGITAL_UNKNOWN` writer - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol\\]. Complete phy_bb_bss_cbw40_dig replaces bits 3:2 with zero or encoding one."]
pub type BSS_CBW_40_DIGITAL_UNKNOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BB_AGC_UPDATE_ENABLE_UNKNOWN` reader - SOURCE\\[ROM_REV0_PHY_AGC,ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_bb_agc_reg_update sets bits 13:11; phy_bb_reg_init independently sets only the low bit."]
pub type BB_AGC_UPDATE_ENABLE_UNKNOWN_R = crate::FieldReader;
#[doc = "Field `BB_AGC_UPDATE_ENABLE_UNKNOWN` writer - SOURCE\\[ROM_REV0_PHY_AGC,ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_bb_agc_reg_update sets bits 13:11; phy_bb_reg_init independently sets only the low bit."]
pub type BB_AGC_UPDATE_ENABLE_UNKNOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MAC_BASEBAND_ENABLE_UNKNOWN` reader - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol\\]. Complete phy_mac_enable_bb sets bit 28 before pulsing WIFI_ENABLE."]
pub type MAC_BASEBAND_ENABLE_UNKNOWN_R = crate::BitReader;
#[doc = "Field `MAC_BASEBAND_ENABLE_UNKNOWN` writer - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol\\]. Complete phy_mac_enable_bb sets bit 28 before pulsing WIFI_ENABLE."]
pub type MAC_BASEBAND_ENABLE_UNKNOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SOURCE\\[BLOB_LIBPHY_REGISTER_CHIPV7_PHY\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. register_chipv7_phy clears bit 0 together with WIFI_ENABLE before taking ownership of the powered PHY."]
    #[inline(always)]
    pub fn cold_start_clear_unknown(&self) -> COLD_START_CLEAR_UNKNOWN_R {
        COLD_START_CLEAR_UNKNOWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL,ROM_REV0_PHY_PBUS, BLOB_LIBPHY_REGISTER_CHIPV7_PHY\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol-and-consumers\\]. Complete phy_wifi_enable_set controls bit 1; phy_pbus_force_mode samples it and register_chipv7_phy initially clears it."]
    #[inline(always)]
    pub fn wifi_enable(&self) -> WIFI_ENABLE_R {
        WIFI_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol\\]. Complete phy_bb_bss_cbw40_dig replaces bits 3:2 with zero or encoding one."]
    #[inline(always)]
    pub fn bss_cbw_40_digital_unknown(&self) -> BSS_CBW_40_DIGITAL_UNKNOWN_R {
        BSS_CBW_40_DIGITAL_UNKNOWN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 11:13 - SOURCE\\[ROM_REV0_PHY_AGC,ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_bb_agc_reg_update sets bits 13:11; phy_bb_reg_init independently sets only the low bit."]
    #[inline(always)]
    pub fn bb_agc_update_enable_unknown(&self) -> BB_AGC_UPDATE_ENABLE_UNKNOWN_R {
        BB_AGC_UPDATE_ENABLE_UNKNOWN_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 28 - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol\\]. Complete phy_mac_enable_bb sets bit 28 before pulsing WIFI_ENABLE."]
    #[inline(always)]
    pub fn mac_baseband_enable_unknown(&self) -> MAC_BASEBAND_ENABLE_UNKNOWN_R {
        MAC_BASEBAND_ENABLE_UNKNOWN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_BB_CFG")
            .field("cold_start_clear_unknown", &self.cold_start_clear_unknown())
            .field("wifi_enable", &self.wifi_enable())
            .field(
                "bss_cbw_40_digital_unknown",
                &self.bss_cbw_40_digital_unknown(),
            )
            .field(
                "bb_agc_update_enable_unknown",
                &self.bb_agc_update_enable_unknown(),
            )
            .field(
                "mac_baseband_enable_unknown",
                &self.mac_baseband_enable_unknown(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SOURCE\\[BLOB_LIBPHY_REGISTER_CHIPV7_PHY\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. register_chipv7_phy clears bit 0 together with WIFI_ENABLE before taking ownership of the powered PHY."]
    #[inline(always)]
    pub fn cold_start_clear_unknown(&mut self) -> COLD_START_CLEAR_UNKNOWN_W<'_, WIFI_BB_CFG_SPEC> {
        COLD_START_CLEAR_UNKNOWN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL,ROM_REV0_PHY_PBUS, BLOB_LIBPHY_REGISTER_CHIPV7_PHY\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol-and-consumers\\]. Complete phy_wifi_enable_set controls bit 1; phy_pbus_force_mode samples it and register_chipv7_phy initially clears it."]
    #[inline(always)]
    pub fn wifi_enable(&mut self) -> WIFI_ENABLE_W<'_, WIFI_BB_CFG_SPEC> {
        WIFI_ENABLE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol\\]. Complete phy_bb_bss_cbw40_dig replaces bits 3:2 with zero or encoding one."]
    #[inline(always)]
    pub fn bss_cbw_40_digital_unknown(
        &mut self,
    ) -> BSS_CBW_40_DIGITAL_UNKNOWN_W<'_, WIFI_BB_CFG_SPEC> {
        BSS_CBW_40_DIGITAL_UNKNOWN_W::new(self, 2)
    }
    #[doc = "Bits 11:13 - SOURCE\\[ROM_REV0_PHY_AGC,ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_bb_agc_reg_update sets bits 13:11; phy_bb_reg_init independently sets only the low bit."]
    #[inline(always)]
    pub fn bb_agc_update_enable_unknown(
        &mut self,
    ) -> BB_AGC_UPDATE_ENABLE_UNKNOWN_W<'_, WIFI_BB_CFG_SPEC> {
        BB_AGC_UPDATE_ENABLE_UNKNOWN_W::new(self, 11)
    }
    #[doc = "Bit 28 - SOURCE\\[ROM_REV0_PHY_FREQUENCY_CHANNEL\\]; CONFIDENCE\\[instruction-exact-semantics-from-symbol\\]. Complete phy_mac_enable_bb sets bit 28 before pulsing WIFI_ENABLE."]
    #[inline(always)]
    pub fn mac_baseband_enable_unknown(
        &mut self,
    ) -> MAC_BASEBAND_ENABLE_UNKNOWN_W<'_, WIFI_BB_CFG_SPEC> {
        MAC_BASEBAND_ENABLE_UNKNOWN_W::new(self, 28)
    }
}
#[doc = "SOURCE\\[S31_MODEM_SYSCON_STRUCT,ROM_REV0_PHY_PBUS,ROM_REV0_PHY_AGC, ROM_REV0_PHY_FREQUENCY_CHANNEL,BLOB_LIBPHY_PHY_BB_INIT\\]; CONFIDENCE\\[mixed-per-field\\]. Wi-Fi baseband configuration; only fields exercised by complete ESP32-S31 ROM/blob bodies are named.\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_bb_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_bb_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_BB_CFG_SPEC;
impl crate::RegisterSpec for WIFI_BB_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_bb_cfg::R`](R) reader structure"]
impl crate::Readable for WIFI_BB_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_bb_cfg::W`](W) writer structure"]
impl crate::Writable for WIFI_BB_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_BB_CFG to value 0"]
impl crate::Resettable for WIFI_BB_CFG_SPEC {}
