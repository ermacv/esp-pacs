#[doc = "Register `FC_07` reader"]
pub type R = crate::R<FC_07_SPEC>;
#[doc = "Register `FC_07` writer"]
pub type W = crate::W<FC_07_SPEC>;
#[doc = "Field `CNT_NUM` reader - 3 ms counter select. 0: 392 us, 1: 682 us, 2: 1.36 ms, 3: 2.72 ms"]
pub type CNT_NUM_R = crate::FieldReader;
#[doc = "Field `CNT_NUM` writer - 3 ms counter select. 0: 392 us, 1: 682 us, 2: 1.36 ms, 3: 2.72 ms"]
pub type CNT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK480_SEL` reader - CLK_480 output time. 0: valid a delay after PLL lock, 1: valid immediately after PLL lock"]
pub type CLK480_SEL_R = crate::BitReader;
#[doc = "Field `CLK480_SEL` writer - CLK_480 output time. 0: valid a delay after PLL lock, 1: valid immediately after PLL lock"]
pub type CLK480_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 3 ms counter select. 0: 392 us, 1: 682 us, 2: 1.36 ms, 3: 2.72 ms"]
    #[inline(always)]
    pub fn cnt_num(&self) -> CNT_NUM_R {
        CNT_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CLK_480 output time. 0: valid a delay after PLL lock, 1: valid immediately after PLL lock"]
    #[inline(always)]
    pub fn clk480_sel(&self) -> CLK480_SEL_R {
        CLK480_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_07")
            .field("cnt_num", &self.cnt_num())
            .field("clk480_sel", &self.clk480_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 3 ms counter select. 0: 392 us, 1: 682 us, 2: 1.36 ms, 3: 2.72 ms"]
    #[inline(always)]
    pub fn cnt_num(&mut self) -> CNT_NUM_W<'_, FC_07_SPEC> {
        CNT_NUM_W::new(self, 0)
    }
    #[doc = "Bit 2 - CLK_480 output time. 0: valid a delay after PLL lock, 1: valid immediately after PLL lock"]
    #[inline(always)]
    pub fn clk480_sel(&mut self) -> CLK480_SEL_W<'_, FC_07_SPEC> {
        CLK480_SEL_W::new(self, 2)
    }
}
#[doc = "3 ms counter and CLK_480 output timing\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_07::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_07::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_07_SPEC;
impl crate::RegisterSpec for FC_07_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_07::R`](R) reader structure"]
impl crate::Readable for FC_07_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_07::W`](W) writer structure"]
impl crate::Writable for FC_07_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_07 to value 0"]
impl crate::Resettable for FC_07_SPEC {}
