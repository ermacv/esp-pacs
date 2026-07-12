#[doc = "Register `REGISTER1_TRANSMITPOLLDEMANDREGISTER` writer"]
pub type W = crate::W<REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC>;
#[doc = "Field `TPD` writer - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 _Current Host Transmit Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the transmission returns to the Suspend state and Bit 2 _TU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the transmission resumes"]
pub type TPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 _Current Host Transmit Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the transmission returns to the Suspend state and Bit 2 _TU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the transmission resumes"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W<'_, REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC> {
        TPD_W::new(self, 0)
    }
}
#[doc = "Used by the host to instruct the DMA to poll the Transmit Descriptor list\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register1_transmitpolldemandregister::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`register1_transmitpolldemandregister::W`](W) writer structure"]
impl crate::Writable for REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER1_TRANSMITPOLLDEMANDREGISTER to value 0"]
impl crate::Resettable for REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC {}
