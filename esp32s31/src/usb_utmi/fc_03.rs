#[doc = "Register `FC_03` reader"]
pub type R = crate::R<FC_03_SPEC>;
#[doc = "Register `FC_03` writer"]
pub type W = crate::W<FC_03_SPEC>;
#[doc = "Field `ADJ_PLL` reader - PLL adjustment"]
pub type ADJ_PLL_R = crate::FieldReader;
#[doc = "Field `ADJ_PLL` writer - PLL adjustment"]
pub type ADJ_PLL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADJ_TXCLK_PHASE` reader - TX clock phase adjustment"]
pub type ADJ_TXCLK_PHASE_R = crate::FieldReader;
#[doc = "Field `ADJ_TXCLK_PHASE` writer - TX clock phase adjustment"]
pub type ADJ_TXCLK_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - PLL adjustment"]
    #[inline(always)]
    pub fn adj_pll(&self) -> ADJ_PLL_R {
        ADJ_PLL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - TX clock phase adjustment"]
    #[inline(always)]
    pub fn adj_txclk_phase(&self) -> ADJ_TXCLK_PHASE_R {
        ADJ_TXCLK_PHASE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_03")
            .field("adj_pll", &self.adj_pll())
            .field("adj_txclk_phase", &self.adj_txclk_phase())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL adjustment"]
    #[inline(always)]
    pub fn adj_pll(&mut self) -> ADJ_PLL_W<'_, FC_03_SPEC> {
        ADJ_PLL_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - TX clock phase adjustment"]
    #[inline(always)]
    pub fn adj_txclk_phase(&mut self) -> ADJ_TXCLK_PHASE_W<'_, FC_03_SPEC> {
        ADJ_TXCLK_PHASE_W::new(self, 4)
    }
}
#[doc = "PLL and TX clock phase\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_03_SPEC;
impl crate::RegisterSpec for FC_03_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_03::R`](R) reader structure"]
impl crate::Readable for FC_03_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_03::W`](W) writer structure"]
impl crate::Writable for FC_03_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_03 to value 0x05"]
impl crate::Resettable for FC_03_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
