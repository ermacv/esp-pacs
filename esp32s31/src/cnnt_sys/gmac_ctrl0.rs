#[doc = "Register `GMAC_CTRL0` reader"]
pub type R = crate::R<GMAC_CTRL0_SPEC>;
#[doc = "Register `GMAC_CTRL0` writer"]
pub type W = crate::W<GMAC_CTRL0_SPEC>;
#[doc = "Field `SBD_FLOWCTRL` reader - "]
pub type SBD_FLOWCTRL_R = crate::BitReader;
#[doc = "Field `SBD_FLOWCTRL` writer - "]
pub type SBD_FLOWCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_INTF_SEL` reader - "]
pub type PHY_INTF_SEL_R = crate::FieldReader;
#[doc = "Field `PHY_INTF_SEL` writer - "]
pub type PHY_INTF_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GMAC_MEM_CLK_FORCE_ON` reader - "]
pub type GMAC_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `GMAC_MEM_CLK_FORCE_ON` writer - "]
pub type GMAC_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMAC_CSYSREQ` reader - "]
pub type GMAC_CSYSREQ_R = crate::BitReader;
#[doc = "Field `GMAC_CSYSREQ` writer - "]
pub type GMAC_CSYSREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sbd_flowctrl(&self) -> SBD_FLOWCTRL_R {
        SBD_FLOWCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn phy_intf_sel(&self) -> PHY_INTF_SEL_R {
        PHY_INTF_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gmac_mem_clk_force_on(&self) -> GMAC_MEM_CLK_FORCE_ON_R {
        GMAC_MEM_CLK_FORCE_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gmac_csysreq(&self) -> GMAC_CSYSREQ_R {
        GMAC_CSYSREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMAC_CTRL0")
            .field("sbd_flowctrl", &self.sbd_flowctrl())
            .field("phy_intf_sel", &self.phy_intf_sel())
            .field("gmac_mem_clk_force_on", &self.gmac_mem_clk_force_on())
            .field("gmac_csysreq", &self.gmac_csysreq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sbd_flowctrl(&mut self) -> SBD_FLOWCTRL_W<'_, GMAC_CTRL0_SPEC> {
        SBD_FLOWCTRL_W::new(self, 1)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn phy_intf_sel(&mut self) -> PHY_INTF_SEL_W<'_, GMAC_CTRL0_SPEC> {
        PHY_INTF_SEL_W::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gmac_mem_clk_force_on(&mut self) -> GMAC_MEM_CLK_FORCE_ON_W<'_, GMAC_CTRL0_SPEC> {
        GMAC_MEM_CLK_FORCE_ON_W::new(self, 5)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gmac_csysreq(&mut self) -> GMAC_CSYSREQ_W<'_, GMAC_CTRL0_SPEC> {
        GMAC_CSYSREQ_W::new(self, 10)
    }
}
#[doc = "GMAC interface and low-power control\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC_CTRL0_SPEC;
impl crate::RegisterSpec for GMAC_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_ctrl0::R`](R) reader structure"]
impl crate::Readable for GMAC_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac_ctrl0::W`](W) writer structure"]
impl crate::Writable for GMAC_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMAC_CTRL0 to value 0"]
impl crate::Resettable for GMAC_CTRL0_SPEC {}
