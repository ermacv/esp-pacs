#[doc = "Register `FC_06` reader"]
pub type R = crate::R<FC_06_SPEC>;
#[doc = "Register `FC_06` writer"]
pub type W = crate::W<FC_06_SPEC>;
#[doc = "Field `LS_PAR_EN` reader - Low-speed mode with parallel enable"]
pub type LS_PAR_EN_R = crate::BitReader;
#[doc = "Field `LS_PAR_EN` writer - Low-speed mode with parallel enable"]
pub type LS_PAR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DET_FSEOP_EN` reader - Full-speed EOP detect enable"]
pub type DET_FSEOP_EN_R = crate::BitReader;
#[doc = "Field `DET_FSEOP_EN` writer - Full-speed EOP detect enable"]
pub type DET_FSEOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_HPHY_LSIE` reader - Preamble disable enable"]
pub type PRE_HPHY_LSIE_R = crate::BitReader;
#[doc = "Field `PRE_HPHY_LSIE` writer - Preamble disable enable"]
pub type PRE_HPHY_LSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS_KPALV_EN` reader - Low-speed mode keep-alive enable"]
pub type LS_KPALV_EN_R = crate::BitReader;
#[doc = "Field `LS_KPALV_EN` writer - Low-speed mode keep-alive enable"]
pub type LS_KPALV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_TX2RX_DLY_CNT_SEL` reader - High-speed bus turn-around time select"]
pub type HS_TX2RX_DLY_CNT_SEL_R = crate::FieldReader;
#[doc = "Field `HS_TX2RX_DLY_CNT_SEL` writer - High-speed bus turn-around time select"]
pub type HS_TX2RX_DLY_CNT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Low-speed mode with parallel enable"]
    #[inline(always)]
    pub fn ls_par_en(&self) -> LS_PAR_EN_R {
        LS_PAR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full-speed EOP detect enable"]
    #[inline(always)]
    pub fn det_fseop_en(&self) -> DET_FSEOP_EN_R {
        DET_FSEOP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Preamble disable enable"]
    #[inline(always)]
    pub fn pre_hphy_lsie(&self) -> PRE_HPHY_LSIE_R {
        PRE_HPHY_LSIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low-speed mode keep-alive enable"]
    #[inline(always)]
    pub fn ls_kpalv_en(&self) -> LS_KPALV_EN_R {
        LS_KPALV_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - High-speed bus turn-around time select"]
    #[inline(always)]
    pub fn hs_tx2rx_dly_cnt_sel(&self) -> HS_TX2RX_DLY_CNT_SEL_R {
        HS_TX2RX_DLY_CNT_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_06")
            .field("ls_par_en", &self.ls_par_en())
            .field("det_fseop_en", &self.det_fseop_en())
            .field("pre_hphy_lsie", &self.pre_hphy_lsie())
            .field("ls_kpalv_en", &self.ls_kpalv_en())
            .field("hs_tx2rx_dly_cnt_sel", &self.hs_tx2rx_dly_cnt_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low-speed mode with parallel enable"]
    #[inline(always)]
    pub fn ls_par_en(&mut self) -> LS_PAR_EN_W<'_, FC_06_SPEC> {
        LS_PAR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Full-speed EOP detect enable"]
    #[inline(always)]
    pub fn det_fseop_en(&mut self) -> DET_FSEOP_EN_W<'_, FC_06_SPEC> {
        DET_FSEOP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Preamble disable enable"]
    #[inline(always)]
    pub fn pre_hphy_lsie(&mut self) -> PRE_HPHY_LSIE_W<'_, FC_06_SPEC> {
        PRE_HPHY_LSIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Low-speed mode keep-alive enable"]
    #[inline(always)]
    pub fn ls_kpalv_en(&mut self) -> LS_KPALV_EN_W<'_, FC_06_SPEC> {
        LS_KPALV_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - High-speed bus turn-around time select"]
    #[inline(always)]
    pub fn hs_tx2rx_dly_cnt_sel(&mut self) -> HS_TX2RX_DLY_CNT_SEL_W<'_, FC_06_SPEC> {
        HS_TX2RX_DLY_CNT_SEL_W::new(self, 4)
    }
}
#[doc = "Low-speed mode and high-speed turn-around\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_06::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_06::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_06_SPEC;
impl crate::RegisterSpec for FC_06_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_06::R`](R) reader structure"]
impl crate::Readable for FC_06_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_06::W`](W) writer structure"]
impl crate::Writable for FC_06_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_06 to value 0x40"]
impl crate::Resettable for FC_06_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
