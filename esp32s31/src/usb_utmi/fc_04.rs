#[doc = "Register `FC_04` reader"]
pub type R = crate::R<FC_04_SPEC>;
#[doc = "Register `FC_04` writer"]
pub type W = crate::W<FC_04_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Test select (output left unconnected)\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_04_SPEC;
impl crate::RegisterSpec for FC_04_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_04::R`](R) reader structure"]
impl crate::Readable for FC_04_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_04::W`](W) writer structure"]
impl crate::Writable for FC_04_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_04 to value 0"]
impl crate::Resettable for FC_04_SPEC {}
