#[doc = "Register `FIB_ENABLE` reader"]
pub type R = crate::R<FIB_ENABLE_SPEC>;
#[doc = "Register `FIB_ENABLE` writer"]
pub type W = crate::W<FIB_ENABLE_SPEC>;
#[doc = "Field `BOD_MODE1_FIB` reader - Brownout detector mode-1 control: 1 leaves it to the analog default, 0 gives it to software (BOD_MODE1_CNTL)"]
pub type BOD_MODE1_FIB_R = crate::BitReader;
#[doc = "Field `BOD_MODE1_FIB` writer - Brownout detector mode-1 control: 1 leaves it to the analog default, 0 gives it to software (BOD_MODE1_CNTL)"]
pub type BOD_MODE1_FIB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Brownout detector mode-1 control: 1 leaves it to the analog default, 0 gives it to software (BOD_MODE1_CNTL)"]
    #[inline(always)]
    pub fn bod_mode1_fib(&self) -> BOD_MODE1_FIB_R {
        BOD_MODE1_FIB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIB_ENABLE")
            .field("bod_mode1_fib", &self.bod_mode1_fib())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Brownout detector mode-1 control: 1 leaves it to the analog default, 0 gives it to software (BOD_MODE1_CNTL)"]
    #[inline(always)]
    pub fn bod_mode1_fib(&mut self) -> BOD_MODE1_FIB_W<'_, FIB_ENABLE_SPEC> {
        BOD_MODE1_FIB_W::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`fib_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fib_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIB_ENABLE_SPEC;
impl crate::RegisterSpec for FIB_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fib_enable::R`](R) reader structure"]
impl crate::Readable for FIB_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fib_enable::W`](W) writer structure"]
impl crate::Writable for FIB_ENABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIB_ENABLE to value 0xffff_ffff"]
impl crate::Resettable for FIB_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
