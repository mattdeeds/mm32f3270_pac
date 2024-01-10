#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `CMDD` reader - CMD complete interrupt mask bit"]
pub type CMDD_R = crate::BitReader;
#[doc = "Field `CMDD` writer - CMD complete interrupt mask bit"]
pub type CMDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATD` reader - DAT complete interrupt mask bit"]
pub type DATD_R = crate::BitReader;
#[doc = "Field `DATD` writer - DAT complete interrupt mask bit"]
pub type DATD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATE` reader - DAT CRC error interrupt mask bit"]
pub type DATE_R = crate::BitReader;
#[doc = "Field `DATE` writer - DAT CRC error interrupt mask bit"]
pub type DATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDE` reader - CMD CRC error interrupt mask bit"]
pub type CMDE_R = crate::BitReader;
#[doc = "Field `CMDE` writer - CMD CRC error interrupt mask bit"]
pub type CMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBD` reader - Multi-block transfer complete interrupt mask bit"]
pub type MBD_R = crate::BitReader;
#[doc = "Field `MBD` writer - Multi-block transfer complete interrupt mask bit"]
pub type MBD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBT` reader - Multi-block transmission timeout interrupt mask bit"]
pub type MBT_R = crate::BitReader;
#[doc = "Field `MBT` writer - Multi-block transmission timeout interrupt mask bit"]
pub type MBT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNT` reader - Command and response Ncr timeout interrupt mask bit"]
pub type CRNT_R = crate::BitReader;
#[doc = "Field `CRNT` writer - Command and response Ncr timeout interrupt mask bit"]
pub type CRNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCE` reader - CRC status error flag interrupt mask bit"]
pub type CRCE_R = crate::BitReader;
#[doc = "Field `CRCE` writer - CRC status error flag interrupt mask bit"]
pub type CRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1` reader - SDIO data1 line interrupt flag/clear bit"]
pub type D1_R = crate::BitReader;
#[doc = "Field `D1` writer - SDIO data1 line interrupt flag/clear bit"]
pub type D1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMD complete interrupt mask bit"]
    #[inline(always)]
    pub fn cmdd(&self) -> CMDD_R {
        CMDD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAT complete interrupt mask bit"]
    #[inline(always)]
    pub fn datd(&self) -> DATD_R {
        DATD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT CRC error interrupt mask bit"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD CRC error interrupt mask bit"]
    #[inline(always)]
    pub fn cmde(&self) -> CMDE_R {
        CMDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-block transfer complete interrupt mask bit"]
    #[inline(always)]
    pub fn mbd(&self) -> MBD_R {
        MBD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-block transmission timeout interrupt mask bit"]
    #[inline(always)]
    pub fn mbt(&self) -> MBT_R {
        MBT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command and response Ncr timeout interrupt mask bit"]
    #[inline(always)]
    pub fn crnt(&self) -> CRNT_R {
        CRNT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC status error flag interrupt mask bit"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO data1 line interrupt flag/clear bit"]
    #[inline(always)]
    pub fn d1(&self) -> D1_R {
        D1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMD complete interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmdd(&mut self) -> CMDD_W<ICR_SPEC> {
        CMDD_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAT complete interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn datd(&mut self) -> DATD_W<ICR_SPEC> {
        DATD_W::new(self, 1)
    }
    #[doc = "Bit 2 - DAT CRC error interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<ICR_SPEC> {
        DATE_W::new(self, 2)
    }
    #[doc = "Bit 3 - CMD CRC error interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmde(&mut self) -> CMDE_W<ICR_SPEC> {
        CMDE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Multi-block transfer complete interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn mbd(&mut self) -> MBD_W<ICR_SPEC> {
        MBD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Multi-block transmission timeout interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn mbt(&mut self) -> MBT_W<ICR_SPEC> {
        MBT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Command and response Ncr timeout interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn crnt(&mut self) -> CRNT_W<ICR_SPEC> {
        CRNT_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC status error flag interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<ICR_SPEC> {
        CRCE_W::new(self, 7)
    }
    #[doc = "Bit 8 - SDIO data1 line interrupt flag/clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn d1(&mut self) -> D1_W<ICR_SPEC> {
        D1_W::new(self, 8)
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
#[doc = "Interrupt clear mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
