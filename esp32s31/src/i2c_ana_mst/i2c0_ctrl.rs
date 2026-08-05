#[doc = "Register `I2C0_CTRL` reader"]
pub type R = crate::R<I2C0_CTRL_SPEC>;
#[doc = "Register `I2C0_CTRL` writer"]
pub type W = crate::W<I2C0_CTRL_SPEC>;
#[doc = "Field `SLAVE_ADDR` reader - Analog block identifier."]
pub type SLAVE_ADDR_R = crate::FieldReader;
#[doc = "Field `SLAVE_ADDR` writer - Analog block identifier."]
pub type SLAVE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `SLAVE_REG_ADDR` reader - Register address within the selected analog block."]
pub type SLAVE_REG_ADDR_R = crate::FieldReader;
#[doc = "Field `SLAVE_REG_ADDR` writer - Register address within the selected analog block."]
pub type SLAVE_REG_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `DATA` reader - Write data or completed read result."]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - Write data or completed read result."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `READ_WRITE` reader - Write transaction when set; read transaction when clear."]
pub type READ_WRITE_R = crate::BitReader;
#[doc = "Field `READ_WRITE` writer - Write transaction when set; read transaction when clear."]
pub type READ_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Hardware transaction busy status."]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - Hardware transaction busy status."]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_OR_RESET` reader - Transaction-start command, or the one-bit host reset word when written without the address/data fields."]
pub type START_OR_RESET_R = crate::BitReader;
#[doc = "Field `START_OR_RESET` writer - Transaction-start command, or the one-bit host reset word when written without the address/data fields."]
pub type START_OR_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Analog block identifier."]
    #[inline(always)]
    pub fn slave_addr(&self) -> SLAVE_ADDR_R {
        SLAVE_ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Register address within the selected analog block."]
    #[inline(always)]
    pub fn slave_reg_addr(&self) -> SLAVE_REG_ADDR_R {
        SLAVE_REG_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write data or completed read result."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Write transaction when set; read transaction when clear."]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Hardware transaction busy status."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transaction-start command, or the one-bit host reset word when written without the address/data fields."]
    #[inline(always)]
    pub fn start_or_reset(&self) -> START_OR_RESET_R {
        START_OR_RESET_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_CTRL")
            .field("slave_addr", &self.slave_addr())
            .field("slave_reg_addr", &self.slave_reg_addr())
            .field("data", &self.data())
            .field("read_write", &self.read_write())
            .field("busy", &self.busy())
            .field("start_or_reset", &self.start_or_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Analog block identifier."]
    #[inline(always)]
    pub fn slave_addr(&mut self) -> SLAVE_ADDR_W<'_, I2C0_CTRL_SPEC> {
        SLAVE_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Register address within the selected analog block."]
    #[inline(always)]
    pub fn slave_reg_addr(&mut self) -> SLAVE_REG_ADDR_W<'_, I2C0_CTRL_SPEC> {
        SLAVE_REG_ADDR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Write data or completed read result."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, I2C0_CTRL_SPEC> {
        DATA_W::new(self, 16)
    }
    #[doc = "Bit 24 - Write transaction when set; read transaction when clear."]
    #[inline(always)]
    pub fn read_write(&mut self) -> READ_WRITE_W<'_, I2C0_CTRL_SPEC> {
        READ_WRITE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Hardware transaction busy status."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<'_, I2C0_CTRL_SPEC> {
        BUSY_W::new(self, 25)
    }
    #[doc = "Bit 26 - Transaction-start command, or the one-bit host reset word when written without the address/data fields."]
    #[inline(always)]
    pub fn start_or_reset(&mut self) -> START_OR_RESET_W<'_, I2C0_CTRL_SPEC> {
        START_OR_RESET_W::new(self, 26)
    }
}
#[doc = "Analog-register I2C host-0 command and completion word. SOURCE\\[ROM_REV0_PHY_I2C\\]; complete phy_i2c_master_reset, phy_chip_i2c_readReg_org and phy_chip_i2c_writeReg bodies.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C0_CTRL_SPEC;
impl crate::RegisterSpec for I2C0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_ctrl::R`](R) reader structure"]
impl crate::Readable for I2C0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c0_ctrl::W`](W) writer structure"]
impl crate::Writable for I2C0_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C0_CTRL to value 0"]
impl crate::Resettable for I2C0_CTRL_SPEC {}
