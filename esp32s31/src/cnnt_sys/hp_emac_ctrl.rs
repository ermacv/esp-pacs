#[doc = "Register `HP_EMAC_CTRL` reader"]
pub type R = crate::R<HP_EMAC_CTRL_SPEC>;
#[doc = "Register `HP_EMAC_CTRL` writer"]
pub type W = crate::W<HP_EMAC_CTRL_SPEC>;
#[doc = "Field `EMAC_USELESS_CLK_EN` reader - "]
pub type EMAC_USELESS_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_USELESS_CLK_EN` writer - "]
pub type EMAC_USELESS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_RST_EN` reader - "]
pub type EMAC_RST_EN_R = crate::BitReader;
#[doc = "Field `EMAC_RST_EN` writer - "]
pub type EMAC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_FORCE_NORST` reader - "]
pub type EMAC_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `EMAC_FORCE_NORST` writer - "]
pub type EMAC_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn emac_useless_clk_en(&self) -> EMAC_USELESS_CLK_EN_R {
        EMAC_USELESS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn emac_rst_en(&self) -> EMAC_RST_EN_R {
        EMAC_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emac_force_norst(&self) -> EMAC_FORCE_NORST_R {
        EMAC_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_EMAC_CTRL")
            .field("emac_useless_clk_en", &self.emac_useless_clk_en())
            .field("emac_rst_en", &self.emac_rst_en())
            .field("emac_force_norst", &self.emac_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn emac_useless_clk_en(&mut self) -> EMAC_USELESS_CLK_EN_W<'_, HP_EMAC_CTRL_SPEC> {
        EMAC_USELESS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn emac_rst_en(&mut self) -> EMAC_RST_EN_W<'_, HP_EMAC_CTRL_SPEC> {
        EMAC_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emac_force_norst(&mut self) -> EMAC_FORCE_NORST_W<'_, HP_EMAC_CTRL_SPEC> {
        EMAC_FORCE_NORST_W::new(self, 2)
    }
}
#[doc = "EMAC reset control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_EMAC_CTRL_SPEC;
impl crate::RegisterSpec for HP_EMAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_emac_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_EMAC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_emac_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_EMAC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_EMAC_CTRL to value 0"]
impl crate::Resettable for HP_EMAC_CTRL_SPEC {}
