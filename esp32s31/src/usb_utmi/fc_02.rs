#[doc = "Register `FC_02` reader"]
pub type R = crate::R<FC_02_SPEC>;
#[doc = "Register `FC_02` writer"]
pub type W = crate::W<FC_02_SPEC>;
#[doc = "Field `ADJ_IREF_RES` reader - Internal bias current. 0: 125 uA, 7: 100 uA, 15: 78 uA"]
pub type ADJ_IREF_RES_R = crate::FieldReader;
#[doc = "Field `ADJ_IREF_RES` writer - Internal bias current. 0: 125 uA, 7: 100 uA, 15: 78 uA"]
pub type ADJ_IREF_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADJ_VSW_HS` reader - Output eye shape. 0: 320 mV, 4: 400 mV, 7: 460 mV"]
pub type ADJ_VSW_HS_R = crate::FieldReader;
#[doc = "Field `ADJ_VSW_HS` writer - Output eye shape. 0: 320 mV, 4: 400 mV, 7: 460 mV"]
pub type ADJ_VSW_HS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Internal bias current. 0: 125 uA, 7: 100 uA, 15: 78 uA"]
    #[inline(always)]
    pub fn adj_iref_res(&self) -> ADJ_IREF_RES_R {
        ADJ_IREF_RES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Output eye shape. 0: 320 mV, 4: 400 mV, 7: 460 mV"]
    #[inline(always)]
    pub fn adj_vsw_hs(&self) -> ADJ_VSW_HS_R {
        ADJ_VSW_HS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_02")
            .field("adj_iref_res", &self.adj_iref_res())
            .field("adj_vsw_hs", &self.adj_vsw_hs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Internal bias current. 0: 125 uA, 7: 100 uA, 15: 78 uA"]
    #[inline(always)]
    pub fn adj_iref_res(&mut self) -> ADJ_IREF_RES_W<'_, FC_02_SPEC> {
        ADJ_IREF_RES_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Output eye shape. 0: 320 mV, 4: 400 mV, 7: 460 mV"]
    #[inline(always)]
    pub fn adj_vsw_hs(&mut self) -> ADJ_VSW_HS_W<'_, FC_02_SPEC> {
        ADJ_VSW_HS_W::new(self, 4)
    }
}
#[doc = "Bias current and output eye shape\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_02_SPEC;
impl crate::RegisterSpec for FC_02_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_02::R`](R) reader structure"]
impl crate::Readable for FC_02_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_02::W`](W) writer structure"]
impl crate::Writable for FC_02_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_02 to value 0x47"]
impl crate::Resettable for FC_02_SPEC {
    const RESET_VALUE: u32 = 0x47;
}
