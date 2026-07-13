#[doc = "Register `HP_EMAC_TX_CTRL` reader"]
pub type R = crate::R<HP_EMAC_TX_CTRL_SPEC>;
#[doc = "Register `HP_EMAC_TX_CTRL` writer"]
pub type W = crate::W<HP_EMAC_TX_CTRL_SPEC>;
#[doc = "Field `EMAC_TX_PAD_CLK_EN` reader - "]
pub type EMAC_TX_PAD_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_TX_PAD_CLK_EN` writer - "]
pub type EMAC_TX_PAD_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_TX_PAD_CLK_INV_EN` reader - "]
pub type EMAC_TX_PAD_CLK_INV_EN_R = crate::BitReader;
#[doc = "Field `EMAC_TX_PAD_CLK_INV_EN` writer - "]
pub type EMAC_TX_PAD_CLK_INV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_TX_CLK_SEL` reader - "]
pub type EMAC_TX_CLK_SEL_R = crate::BitReader;
#[doc = "Field `EMAC_TX_CLK_SEL` writer - "]
pub type EMAC_TX_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_TX_180_CLK_EN` reader - "]
pub type EMAC_TX_180_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_TX_180_CLK_EN` writer - "]
pub type EMAC_TX_180_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn emac_tx_pad_clk_en(&self) -> EMAC_TX_PAD_CLK_EN_R {
        EMAC_TX_PAD_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn emac_tx_pad_clk_inv_en(&self) -> EMAC_TX_PAD_CLK_INV_EN_R {
        EMAC_TX_PAD_CLK_INV_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emac_tx_clk_sel(&self) -> EMAC_TX_CLK_SEL_R {
        EMAC_TX_CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn emac_tx_180_clk_en(&self) -> EMAC_TX_180_CLK_EN_R {
        EMAC_TX_180_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_EMAC_TX_CTRL")
            .field("emac_tx_pad_clk_en", &self.emac_tx_pad_clk_en())
            .field("emac_tx_pad_clk_inv_en", &self.emac_tx_pad_clk_inv_en())
            .field("emac_tx_clk_sel", &self.emac_tx_clk_sel())
            .field("emac_tx_180_clk_en", &self.emac_tx_180_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn emac_tx_pad_clk_en(&mut self) -> EMAC_TX_PAD_CLK_EN_W<'_, HP_EMAC_TX_CTRL_SPEC> {
        EMAC_TX_PAD_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn emac_tx_pad_clk_inv_en(&mut self) -> EMAC_TX_PAD_CLK_INV_EN_W<'_, HP_EMAC_TX_CTRL_SPEC> {
        EMAC_TX_PAD_CLK_INV_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emac_tx_clk_sel(&mut self) -> EMAC_TX_CLK_SEL_W<'_, HP_EMAC_TX_CTRL_SPEC> {
        EMAC_TX_CLK_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn emac_tx_180_clk_en(&mut self) -> EMAC_TX_180_CLK_EN_W<'_, HP_EMAC_TX_CTRL_SPEC> {
        EMAC_TX_180_CLK_EN_W::new(self, 3)
    }
}
#[doc = "EMAC transmit clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_emac_tx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_emac_tx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_EMAC_TX_CTRL_SPEC;
impl crate::RegisterSpec for HP_EMAC_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_emac_tx_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_EMAC_TX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_emac_tx_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_EMAC_TX_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_EMAC_TX_CTRL to value 0"]
impl crate::Resettable for HP_EMAC_TX_CTRL_SPEC {}
