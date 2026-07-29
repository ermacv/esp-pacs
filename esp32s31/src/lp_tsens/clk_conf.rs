#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "Field `CLK_EN` reader - SOURCE\\[S31_ESP_PACS_BASE_SVD,BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[exact-field-and-instruction-use\\]. Complete phy_tsens_read_init sets this bit in its first fresh RMW operation."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - SOURCE\\[S31_ESP_PACS_BASE_SVD,BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[exact-field-and-instruction-use\\]. Complete phy_tsens_read_init sets this bit in its first fresh RMW operation."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CONVERSION_ENABLE_UNKNOWN` reader - SOURCE\\[BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_tsens_read_init sets bit 9 in its fourth fresh RMW operation; the electrical meaning is not assigned by the blob."]
pub type PHY_CONVERSION_ENABLE_UNKNOWN_R = crate::BitReader;
#[doc = "Field `PHY_CONVERSION_ENABLE_UNKNOWN` writer - SOURCE\\[BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_tsens_read_init sets bit 9 in its fourth fresh RMW operation; the electrical meaning is not assigned by the blob."]
pub type PHY_CONVERSION_ENABLE_UNKNOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_READOUT_ENABLE_UNKNOWN` reader - SOURCE\\[BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_tsens_read_init sets bit 23 in its third fresh RMW operation; the electrical meaning is not assigned by the blob."]
pub type PHY_READOUT_ENABLE_UNKNOWN_R = crate::BitReader;
#[doc = "Field `PHY_READOUT_ENABLE_UNKNOWN` writer - SOURCE\\[BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_tsens_read_init sets bit 23 in its third fresh RMW operation; the electrical meaning is not assigned by the blob."]
pub type PHY_READOUT_ENABLE_UNKNOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SOURCE\\[S31_ESP_PACS_BASE_SVD,BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[exact-field-and-instruction-use\\]. Complete phy_tsens_read_init sets this bit in its first fresh RMW operation."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - SOURCE\\[BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_tsens_read_init sets bit 9 in its fourth fresh RMW operation; the electrical meaning is not assigned by the blob."]
    #[inline(always)]
    pub fn phy_conversion_enable_unknown(&self) -> PHY_CONVERSION_ENABLE_UNKNOWN_R {
        PHY_CONVERSION_ENABLE_UNKNOWN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23 - SOURCE\\[BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_tsens_read_init sets bit 23 in its third fresh RMW operation; the electrical meaning is not assigned by the blob."]
    #[inline(always)]
    pub fn phy_readout_enable_unknown(&self) -> PHY_READOUT_ENABLE_UNKNOWN_R {
        PHY_READOUT_ENABLE_UNKNOWN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("clk_en", &self.clk_en())
            .field(
                "phy_readout_enable_unknown",
                &self.phy_readout_enable_unknown(),
            )
            .field(
                "phy_conversion_enable_unknown",
                &self.phy_conversion_enable_unknown(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SOURCE\\[S31_ESP_PACS_BASE_SVD,BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[exact-field-and-instruction-use\\]. Complete phy_tsens_read_init sets this bit in its first fresh RMW operation."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, CLK_CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 9 - SOURCE\\[BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_tsens_read_init sets bit 9 in its fourth fresh RMW operation; the electrical meaning is not assigned by the blob."]
    #[inline(always)]
    pub fn phy_conversion_enable_unknown(
        &mut self,
    ) -> PHY_CONVERSION_ENABLE_UNKNOWN_W<'_, CLK_CONF_SPEC> {
        PHY_CONVERSION_ENABLE_UNKNOWN_W::new(self, 9)
    }
    #[doc = "Bit 23 - SOURCE\\[BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[instruction-exact-semantics-unknown\\]. Complete phy_tsens_read_init sets bit 23 in its third fresh RMW operation; the electrical meaning is not assigned by the blob."]
    #[inline(always)]
    pub fn phy_readout_enable_unknown(
        &mut self,
    ) -> PHY_READOUT_ENABLE_UNKNOWN_W<'_, CLK_CONF_SPEC> {
        PHY_READOUT_ENABLE_UNKNOWN_W::new(self, 23)
    }
}
#[doc = "SOURCE\\[S31_ESP_PACS_BASE_SVD,BLOB_LIBPHY_PHY_TSENS_READ_INIT\\]; CONFIDENCE\\[exact-register-and-instruction-use\\]. LP temperature-sensor regbank and read-path control. Complete pinned libphy.a\\[phy_tsens.o\\]::phy_tsens_read_init performs three independent fresh RMW operations setting bits 0, 23 and 9, in that order.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CONF to value 0"]
impl crate::Resettable for CLK_CONF_SPEC {}
