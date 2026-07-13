#[doc = "Register `HP_EMAC_REF_CTRL` reader"]
pub type R = crate::R<HP_EMAC_REF_CTRL_SPEC>;
#[doc = "Register `HP_EMAC_REF_CTRL` writer"]
pub type W = crate::W<HP_EMAC_REF_CTRL_SPEC>;
#[doc = "Field `EMAC_REF_CLK_SEL` reader - Reference clock source: 0 = MPLL 500 MHz, 1 = CPLL 300 MHz"]
pub type EMAC_REF_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `EMAC_REF_CLK_SEL` writer - Reference clock source: 0 = MPLL 500 MHz, 1 = CPLL 300 MHz"]
pub type EMAC_REF_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMAC_REF_CLK_EN` reader - "]
pub type EMAC_REF_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_REF_CLK_EN` writer - "]
pub type EMAC_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_REF_50M_PAD_OUT_CLK_EN` reader - "]
pub type EMAC_REF_50M_PAD_OUT_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_REF_50M_PAD_OUT_CLK_EN` writer - "]
pub type EMAC_REF_50M_PAD_OUT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_REF_CLK_DIV_NUM` reader - "]
pub type EMAC_REF_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `EMAC_REF_CLK_DIV_NUM` writer - "]
pub type EMAC_REF_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Reference clock source: 0 = MPLL 500 MHz, 1 = CPLL 300 MHz"]
    #[inline(always)]
    pub fn emac_ref_clk_sel(&self) -> EMAC_REF_CLK_SEL_R {
        EMAC_REF_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emac_ref_clk_en(&self) -> EMAC_REF_CLK_EN_R {
        EMAC_REF_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn emac_ref_50m_pad_out_clk_en(&self) -> EMAC_REF_50M_PAD_OUT_CLK_EN_R {
        EMAC_REF_50M_PAD_OUT_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn emac_ref_clk_div_num(&self) -> EMAC_REF_CLK_DIV_NUM_R {
        EMAC_REF_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_EMAC_REF_CTRL")
            .field("emac_ref_clk_sel", &self.emac_ref_clk_sel())
            .field("emac_ref_clk_en", &self.emac_ref_clk_en())
            .field(
                "emac_ref_50m_pad_out_clk_en",
                &self.emac_ref_50m_pad_out_clk_en(),
            )
            .field("emac_ref_clk_div_num", &self.emac_ref_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Reference clock source: 0 = MPLL 500 MHz, 1 = CPLL 300 MHz"]
    #[inline(always)]
    pub fn emac_ref_clk_sel(&mut self) -> EMAC_REF_CLK_SEL_W<'_, HP_EMAC_REF_CTRL_SPEC> {
        EMAC_REF_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emac_ref_clk_en(&mut self) -> EMAC_REF_CLK_EN_W<'_, HP_EMAC_REF_CTRL_SPEC> {
        EMAC_REF_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn emac_ref_50m_pad_out_clk_en(
        &mut self,
    ) -> EMAC_REF_50M_PAD_OUT_CLK_EN_W<'_, HP_EMAC_REF_CTRL_SPEC> {
        EMAC_REF_50M_PAD_OUT_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn emac_ref_clk_div_num(&mut self) -> EMAC_REF_CLK_DIV_NUM_W<'_, HP_EMAC_REF_CTRL_SPEC> {
        EMAC_REF_CLK_DIV_NUM_W::new(self, 8)
    }
}
#[doc = "EMAC reference clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_ref_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_ref_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_EMAC_REF_CTRL_SPEC;
impl crate::RegisterSpec for HP_EMAC_REF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_emac_ref_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_EMAC_REF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_emac_ref_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_EMAC_REF_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_EMAC_REF_CTRL to value 0"]
impl crate::Resettable for HP_EMAC_REF_CTRL_SPEC {}
