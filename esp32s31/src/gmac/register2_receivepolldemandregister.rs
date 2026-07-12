#[doc = "Register `REGISTER2_RECEIVEPOLLDEMANDREGISTER` writer"]
pub type W = crate::W<REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC>;
#[doc = "Field `RPD` writer - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 _Current Host Receive Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the reception returns to the Suspended state and Bit 7 _RU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the Rx DMA returns to the active state"]
pub type RPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 _Current Host Receive Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the reception returns to the Suspended state and Bit 7 _RU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the Rx DMA returns to the active state"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W<'_, REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC> {
        RPD_W::new(self, 0)
    }
}
#[doc = "Used by the host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register2_receivepolldemandregister::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`register2_receivepolldemandregister::W`](W) writer structure"]
impl crate::Writable for REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER2_RECEIVEPOLLDEMANDREGISTER to value 0"]
impl crate::Resettable for REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC {}
