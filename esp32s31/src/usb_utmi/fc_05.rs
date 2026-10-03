#[doc = "Register `FC_05` reader"]
pub type R = crate::R<FC_05_SPEC>;
#[doc = "Register `FC_05` writer"]
pub type W = crate::W<FC_05_SPEC>;
#[doc = "Field `RXGAP_FIX_EN` reader - RX gap fix enable"]
pub type RXGAP_FIX_EN_R = crate::BitReader;
#[doc = "Field `RXGAP_FIX_EN` writer - RX gap fix enable"]
pub type RXGAP_FIX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTER_SEL` reader - SIE input sample enable"]
pub type COUNTER_SEL_R = crate::BitReader;
#[doc = "Field `COUNTER_SEL` writer - SIE input sample enable"]
pub type COUNTER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SEL` reader - CLK60_30 source select"]
pub type CLK_SEL_R = crate::BitReader;
#[doc = "Field `CLK_SEL` writer - CLK60_30 source select"]
pub type CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_MODE_SEL` reader - PHY mode select"]
pub type PHY_MODE_SEL_R = crate::BitReader;
#[doc = "Field `PHY_MODE_SEL` writer - PHY mode select"]
pub type PHY_MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX gap fix enable"]
    #[inline(always)]
    pub fn rxgap_fix_en(&self) -> RXGAP_FIX_EN_R {
        RXGAP_FIX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SIE input sample enable"]
    #[inline(always)]
    pub fn counter_sel(&self) -> COUNTER_SEL_R {
        COUNTER_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLK60_30 source select"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PHY mode select"]
    #[inline(always)]
    pub fn phy_mode_sel(&self) -> PHY_MODE_SEL_R {
        PHY_MODE_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_05")
            .field("rxgap_fix_en", &self.rxgap_fix_en())
            .field("counter_sel", &self.counter_sel())
            .field("clk_sel", &self.clk_sel())
            .field("phy_mode_sel", &self.phy_mode_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RX gap fix enable"]
    #[inline(always)]
    pub fn rxgap_fix_en(&mut self) -> RXGAP_FIX_EN_W<'_, FC_05_SPEC> {
        RXGAP_FIX_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SIE input sample enable"]
    #[inline(always)]
    pub fn counter_sel(&mut self) -> COUNTER_SEL_W<'_, FC_05_SPEC> {
        COUNTER_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - CLK60_30 source select"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<'_, FC_05_SPEC> {
        CLK_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - PHY mode select"]
    #[inline(always)]
    pub fn phy_mode_sel(&mut self) -> PHY_MODE_SEL_W<'_, FC_05_SPEC> {
        PHY_MODE_SEL_W::new(self, 3)
    }
}
#[doc = "Receive and clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_05::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_05::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_05_SPEC;
impl crate::RegisterSpec for FC_05_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_05::R`](R) reader structure"]
impl crate::Readable for FC_05_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_05::W`](W) writer structure"]
impl crate::Writable for FC_05_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_05 to value 0x01"]
impl crate::Resettable for FC_05_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
