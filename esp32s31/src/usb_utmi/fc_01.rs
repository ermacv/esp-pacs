#[doc = "Register `FC_01` reader"]
pub type R = crate::R<FC_01_SPEC>;
#[doc = "Register `FC_01` writer"]
pub type W = crate::W<FC_01_SPEC>;
#[doc = "Field `ADJ_VREF_SQ` reader - Squelch detection threshold voltage. 0: 92 mV, 2: 124 mV, 3: 152 mV"]
pub type ADJ_VREF_SQ_R = crate::FieldReader;
#[doc = "Field `ADJ_VREF_SQ` writer - Squelch detection threshold voltage. 0: 92 mV, 2: 124 mV, 3: 152 mV"]
pub type ADJ_VREF_SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADJ_PW_HS` reader - High-speed output swing. 1: 100 mV, 3: 200 mV, 7: 300 mV, 15: 400 mV"]
pub type ADJ_PW_HS_R = crate::FieldReader;
#[doc = "Field `ADJ_PW_HS` writer - High-speed output swing. 1: 100 mV, 3: 200 mV, 7: 300 mV, 15: 400 mV"]
pub type ADJ_PW_HS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Squelch detection threshold voltage. 0: 92 mV, 2: 124 mV, 3: 152 mV"]
    #[inline(always)]
    pub fn adj_vref_sq(&self) -> ADJ_VREF_SQ_R {
        ADJ_VREF_SQ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - High-speed output swing. 1: 100 mV, 3: 200 mV, 7: 300 mV, 15: 400 mV"]
    #[inline(always)]
    pub fn adj_pw_hs(&self) -> ADJ_PW_HS_R {
        ADJ_PW_HS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_01")
            .field("adj_vref_sq", &self.adj_vref_sq())
            .field("adj_pw_hs", &self.adj_pw_hs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Squelch detection threshold voltage. 0: 92 mV, 2: 124 mV, 3: 152 mV"]
    #[inline(always)]
    pub fn adj_vref_sq(&mut self) -> ADJ_VREF_SQ_W<'_, FC_01_SPEC> {
        ADJ_VREF_SQ_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - High-speed output swing. 1: 100 mV, 3: 200 mV, 7: 300 mV, 15: 400 mV"]
    #[inline(always)]
    pub fn adj_pw_hs(&mut self) -> ADJ_PW_HS_W<'_, FC_01_SPEC> {
        ADJ_PW_HS_W::new(self, 4)
    }
}
#[doc = "Squelch threshold and high-speed output swing\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_01_SPEC;
impl crate::RegisterSpec for FC_01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_01::R`](R) reader structure"]
impl crate::Readable for FC_01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_01::W`](W) writer structure"]
impl crate::Writable for FC_01_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_01 to value 0xf2"]
impl crate::Resettable for FC_01_SPEC {
    const RESET_VALUE: u32 = 0xf2;
}
