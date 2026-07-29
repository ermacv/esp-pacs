#[doc = "Register `RTC_SAR2_PWDET_CCT` reader"]
pub type R = crate::R<RTC_SAR2_PWDET_CCT_SPEC>;
#[doc = "Register `RTC_SAR2_PWDET_CCT` writer"]
pub type W = crate::W<RTC_SAR2_PWDET_CCT_SPEC>;
#[doc = "Field `RTC_SAR2_PWDET_CCT` reader - SOURCE\\[S31_ESP_PACS_BASE_SVD,ROM_REV0_PHY_POWER_DETECTOR\\]; CONFIDENCE\\[exact-field-and-instruction-use\\]. Three-bit SAR2 power detector circuit encoding. Complete ESP32-S31 rev0 ROM uses values 4 for initialization/enabling and 2 for TX-calibration debug mode."]
pub type RTC_SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `RTC_SAR2_PWDET_CCT` writer - SOURCE\\[S31_ESP_PACS_BASE_SVD,ROM_REV0_PHY_POWER_DETECTOR\\]; CONFIDENCE\\[exact-field-and-instruction-use\\]. Three-bit SAR2 power detector circuit encoding. Complete ESP32-S31 rev0 ROM uses values 4 for initialization/enabling and 2 for TX-calibration debug mode."]
pub type RTC_SAR2_PWDET_CCT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SOURCE\\[S31_ESP_PACS_BASE_SVD,ROM_REV0_PHY_POWER_DETECTOR\\]; CONFIDENCE\\[exact-field-and-instruction-use\\]. Three-bit SAR2 power detector circuit encoding. Complete ESP32-S31 rev0 ROM uses values 4 for initialization/enabling and 2 for TX-calibration debug mode."]
    #[inline(always)]
    pub fn rtc_sar2_pwdet_cct(&self) -> RTC_SAR2_PWDET_CCT_R {
        RTC_SAR2_PWDET_CCT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_SAR2_PWDET_CCT")
            .field("rtc_sar2_pwdet_cct", &self.rtc_sar2_pwdet_cct())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SOURCE\\[S31_ESP_PACS_BASE_SVD,ROM_REV0_PHY_POWER_DETECTOR\\]; CONFIDENCE\\[exact-field-and-instruction-use\\]. Three-bit SAR2 power detector circuit encoding. Complete ESP32-S31 rev0 ROM uses values 4 for initialization/enabling and 2 for TX-calibration debug mode."]
    #[inline(always)]
    pub fn rtc_sar2_pwdet_cct(&mut self) -> RTC_SAR2_PWDET_CCT_W<'_, RTC_SAR2_PWDET_CCT_SPEC> {
        RTC_SAR2_PWDET_CCT_W::new(self, 0)
    }
}
#[doc = "SOURCE\\[S31_ESP_PACS_BASE_SVD,ROM_REV0_PHY_POWER_DETECTOR\\]; CONFIDENCE\\[exact-register-and-instruction-use\\]. LP always-on SAR2 power detector circuit control. Complete ESP32-S31 rev0 ROM phy_pwdet_reg_init and phy_pwdet_sar2_init select encoding 4; complete phy_txcal_debuge_mode_ selects encoding 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_sar2_pwdet_cct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_sar2_pwdet_cct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_SAR2_PWDET_CCT_SPEC;
impl crate::RegisterSpec for RTC_SAR2_PWDET_CCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_sar2_pwdet_cct::R`](R) reader structure"]
impl crate::Readable for RTC_SAR2_PWDET_CCT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_sar2_pwdet_cct::W`](W) writer structure"]
impl crate::Writable for RTC_SAR2_PWDET_CCT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_SAR2_PWDET_CCT to value 0"]
impl crate::Resettable for RTC_SAR2_PWDET_CCT_SPEC {}
