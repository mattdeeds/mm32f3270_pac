#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `MASTER` reader - This bit controls whether the DW_apb_i2c master is enabled"]
pub type MASTER_R = crate::BitReader;
#[doc = "Field `MASTER` writer - This bit controls whether the DW_apb_i2c master is enabled"]
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - These bits control at which speed the DW_apb_i2c operates"]
pub type SPEED_R = crate::FieldReader;
#[doc = "Field `SPEED` writer - These bits control at which speed the DW_apb_i2c operates"]
pub type SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLAVE10` reader - When acting as a alsve"]
pub type SLAVE10_R = crate::BitReader;
#[doc = "Field `SLAVE10` writer - When acting as a alsve"]
pub type SLAVE10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER10` reader - Address mode when acting as a master"]
pub type MASTER10_R = crate::BitReader;
#[doc = "Field `MASTER10` writer - Address mode when acting as a master"]
pub type MASTER10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPEN` reader - Determines whether RESTART comdtions may be sent when acting as a master"]
pub type REPEN_R = crate::BitReader;
#[doc = "Field `REPEN` writer - Determines whether RESTART comdtions may be sent when acting as a master"]
pub type REPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISSLAVE` reader - This bit controls whether I2C has its slave diabled"]
pub type DISSLAVE_R = crate::BitReader;
#[doc = "Field `DISSLAVE` writer - This bit controls whether I2C has its slave diabled"]
pub type DISSLAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPINT` reader - STOP_DET_IFADDRESSED"]
pub type STOPINT_R = crate::BitReader;
#[doc = "Field `STOPINT` writer - STOP_DET_IFADDRESSED"]
pub type STOPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPINT` reader - This bit controls the generation of the TX_EMPTY interrupt"]
pub type EMPINT_R = crate::BitReader;
#[doc = "Field `EMPINT` writer - This bit controls the generation of the TX_EMPTY interrupt"]
pub type EMPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Whether to generate a STOP signal after sending or receiving"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Whether to generate a STOP signal after sending or receiving"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART` reader - Whether to generate a RESTART signal after sending or receiving"]
pub type RESTART_R = crate::BitReader;
#[doc = "Field `RESTART` writer - Whether to generate a RESTART signal after sending or receiving"]
pub type RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_TX_ABRT_DIS` reader - when acting as a slave"]
pub type SLV_TX_ABRT_DIS_R = crate::BitReader;
#[doc = "Field `SLV_TX_ABRT_DIS` writer - when acting as a slave"]
pub type SLV_TX_ABRT_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - When acting as a alsve"]
    #[inline(always)]
    pub fn slave10(&self) -> SLAVE10_R {
        SLAVE10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Address mode when acting as a master"]
    #[inline(always)]
    pub fn master10(&self) -> MASTER10_R {
        MASTER10_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Determines whether RESTART comdtions may be sent when acting as a master"]
    #[inline(always)]
    pub fn repen(&self) -> REPEN_R {
        REPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave diabled"]
    #[inline(always)]
    pub fn disslave(&self) -> DISSLAVE_R {
        DISSLAVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    pub fn stopint(&self) -> STOPINT_R {
        STOPINT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn empint(&self) -> EMPINT_R {
        EMPINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Whether to generate a STOP signal after sending or receiving"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Whether to generate a RESTART signal after sending or receiving"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - when acting as a slave"]
    #[inline(always)]
    pub fn slv_tx_abrt_dis(&self) -> SLV_TX_ABRT_DIS_R {
        SLV_TX_ABRT_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<CR_SPEC> {
        MASTER_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<CR_SPEC> {
        SPEED_W::new(self, 1)
    }
    #[doc = "Bit 3 - When acting as a alsve"]
    #[inline(always)]
    #[must_use]
    pub fn slave10(&mut self) -> SLAVE10_W<CR_SPEC> {
        SLAVE10_W::new(self, 3)
    }
    #[doc = "Bit 4 - Address mode when acting as a master"]
    #[inline(always)]
    #[must_use]
    pub fn master10(&mut self) -> MASTER10_W<CR_SPEC> {
        MASTER10_W::new(self, 4)
    }
    #[doc = "Bit 5 - Determines whether RESTART comdtions may be sent when acting as a master"]
    #[inline(always)]
    #[must_use]
    pub fn repen(&mut self) -> REPEN_W<CR_SPEC> {
        REPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave diabled"]
    #[inline(always)]
    #[must_use]
    pub fn disslave(&mut self) -> DISSLAVE_W<CR_SPEC> {
        DISSLAVE_W::new(self, 6)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    #[must_use]
    pub fn stopint(&mut self) -> STOPINT_W<CR_SPEC> {
        STOPINT_W::new(self, 7)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn empint(&mut self) -> EMPINT_W<CR_SPEC> {
        EMPINT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Whether to generate a STOP signal after sending or receiving"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR_SPEC> {
        STOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Whether to generate a RESTART signal after sending or receiving"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<CR_SPEC> {
        RESTART_W::new(self, 10)
    }
    #[doc = "Bit 11 - when acting as a slave"]
    #[inline(always)]
    #[must_use]
    pub fn slv_tx_abrt_dis(&mut self) -> SLV_TX_ABRT_DIS_W<CR_SPEC> {
        SLV_TX_ABRT_DIS_W::new(self, 11)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x7f"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
