#[doc = "Register `FC_00` reader"]
pub type R = crate::R<FC_00_SPEC>;
#[doc = "Register `FC_00` writer"]
pub type W = crate::W<FC_00_SPEC>;
#[doc = "Field `CLK_GATE_RX` reader - Clock gating for RX. 0: lower power consumption, 1: lowest power consumption, 2: normal power consumption"]
pub type CLK_GATE_RX_R = crate::FieldReader;
#[doc = "Field `CLK_GATE_RX` writer - Clock gating for RX. 0: lower power consumption, 1: lowest power consumption, 2: normal power consumption"]
pub type CLK_GATE_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_GATE_TX` reader - Clock gating for TX. 0: low power consumption, 1: normal power consumption"]
pub type CLK_GATE_TX_R = crate::BitReader;
#[doc = "Field `CLK_GATE_TX` writer - Clock gating for TX. 0: low power consumption, 1: normal power consumption"]
pub type CLK_GATE_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_RES_FS` reader - Fine tune of the 45 ohm full-speed termination resistor (reserved)"]
pub type ADJ_RES_FS_R = crate::FieldReader;
#[doc = "Field `ADJ_RES_FS` writer - Fine tune of the 45 ohm full-speed termination resistor (reserved)"]
pub type ADJ_RES_FS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADJ_RES_HS` reader - Fine tune of the 45 ohm high-speed termination resistor. 0: 40 ohm, 4: 45 ohm, 6: 50 ohm"]
pub type ADJ_RES_HS_R = crate::FieldReader;
#[doc = "Field `ADJ_RES_HS` writer - Fine tune of the 45 ohm high-speed termination resistor. 0: 40 ohm, 4: 45 ohm, 6: 50 ohm"]
pub type ADJ_RES_HS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Clock gating for RX. 0: lower power consumption, 1: lowest power consumption, 2: normal power consumption"]
    #[inline(always)]
    pub fn clk_gate_rx(&self) -> CLK_GATE_RX_R {
        CLK_GATE_RX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Clock gating for TX. 0: low power consumption, 1: normal power consumption"]
    #[inline(always)]
    pub fn clk_gate_tx(&self) -> CLK_GATE_TX_R {
        CLK_GATE_TX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Fine tune of the 45 ohm full-speed termination resistor (reserved)"]
    #[inline(always)]
    pub fn adj_res_fs(&self) -> ADJ_RES_FS_R {
        ADJ_RES_FS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Fine tune of the 45 ohm high-speed termination resistor. 0: 40 ohm, 4: 45 ohm, 6: 50 ohm"]
    #[inline(always)]
    pub fn adj_res_hs(&self) -> ADJ_RES_HS_R {
        ADJ_RES_HS_R::new(((self.bits >> 5) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_00")
            .field("clk_gate_rx", &self.clk_gate_rx())
            .field("clk_gate_tx", &self.clk_gate_tx())
            .field("adj_res_fs", &self.adj_res_fs())
            .field("adj_res_hs", &self.adj_res_hs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock gating for RX. 0: lower power consumption, 1: lowest power consumption, 2: normal power consumption"]
    #[inline(always)]
    pub fn clk_gate_rx(&mut self) -> CLK_GATE_RX_W<'_, FC_00_SPEC> {
        CLK_GATE_RX_W::new(self, 0)
    }
    #[doc = "Bit 2 - Clock gating for TX. 0: low power consumption, 1: normal power consumption"]
    #[inline(always)]
    pub fn clk_gate_tx(&mut self) -> CLK_GATE_TX_W<'_, FC_00_SPEC> {
        CLK_GATE_TX_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Fine tune of the 45 ohm full-speed termination resistor (reserved)"]
    #[inline(always)]
    pub fn adj_res_fs(&mut self) -> ADJ_RES_FS_W<'_, FC_00_SPEC> {
        ADJ_RES_FS_W::new(self, 3)
    }
    #[doc = "Bits 5:7 - Fine tune of the 45 ohm high-speed termination resistor. 0: 40 ohm, 4: 45 ohm, 6: 50 ohm"]
    #[inline(always)]
    pub fn adj_res_hs(&mut self) -> ADJ_RES_HS_W<'_, FC_00_SPEC> {
        ADJ_RES_HS_W::new(self, 5)
    }
}
#[doc = "Clock gating and termination resistor control\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_00_SPEC;
impl crate::RegisterSpec for FC_00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_00::R`](R) reader structure"]
impl crate::Readable for FC_00_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_00::W`](W) writer structure"]
impl crate::Writable for FC_00_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_00 to value 0x80"]
impl crate::Resettable for FC_00_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
