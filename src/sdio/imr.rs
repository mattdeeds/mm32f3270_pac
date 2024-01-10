#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Field `CMDD` reader - CMD done interrupt mask"]
pub type CMDD_R = crate::BitReader;
#[doc = "Field `CMDD` writer - CMD done interrupt mask"]
pub type CMDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATD` reader - CMD done interrupt mask"]
pub type DATD_R = crate::BitReader;
#[doc = "Field `DATD` writer - CMD done interrupt mask"]
pub type DATD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATE` reader - DAT CRC error interrupt mask"]
pub type DATE_R = crate::BitReader;
#[doc = "Field `DATE` writer - DAT CRC error interrupt mask"]
pub type DATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDE` reader - CMD CRC error interrupt mask"]
pub type CMDE_R = crate::BitReader;
#[doc = "Field `CMDE` writer - CMD CRC error interrupt mask"]
pub type CMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBD` reader - Multi Block done interrupt mask"]
pub type MBD_R = crate::BitReader;
#[doc = "Field `MBD` writer - Multi Block done interrupt mask"]
pub type MBD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBT` reader - Multi Block Timeout interrupt mask"]
pub type MBT_R = crate::BitReader;
#[doc = "Field `MBT` writer - Multi Block Timeout interrupt mask"]
pub type MBT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNT` reader - Cmd and Resp Ncr Timeout interrupt mask"]
pub type CRNT_R = crate::BitReader;
#[doc = "Field `CRNT` writer - Cmd and Resp Ncr Timeout interrupt mask"]
pub type CRNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCE` reader - CRC status token err interrupt mask"]
pub type CRCE_R = crate::BitReader;
#[doc = "Field `CRCE` writer - CRC status token err interrupt mask"]
pub type CRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1L` reader - SDIO data1 line interrupt mask"]
pub type D1L_R = crate::BitReader;
#[doc = "Field `D1L` writer - SDIO data1 line interrupt mask"]
pub type D1L_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMD done interrupt mask"]
    #[inline(always)]
    pub fn cmdd(&self) -> CMDD_R {
        CMDD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD done interrupt mask"]
    #[inline(always)]
    pub fn datd(&self) -> DATD_R {
        DATD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT CRC error interrupt mask"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD CRC error interrupt mask"]
    #[inline(always)]
    pub fn cmde(&self) -> CMDE_R {
        CMDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi Block done interrupt mask"]
    #[inline(always)]
    pub fn mbd(&self) -> MBD_R {
        MBD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi Block Timeout interrupt mask"]
    #[inline(always)]
    pub fn mbt(&self) -> MBT_R {
        MBT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Cmd and Resp Ncr Timeout interrupt mask"]
    #[inline(always)]
    pub fn crnt(&self) -> CRNT_R {
        CRNT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC status token err interrupt mask"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO data1 line interrupt mask"]
    #[inline(always)]
    pub fn d1l(&self) -> D1L_R {
        D1L_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMD done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn cmdd(&mut self) -> CMDD_W<IMR_SPEC> {
        CMDD_W::new(self, 0)
    }
    #[doc = "Bit 1 - CMD done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn datd(&mut self) -> DATD_W<IMR_SPEC> {
        DATD_W::new(self, 1)
    }
    #[doc = "Bit 2 - DAT CRC error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<IMR_SPEC> {
        DATE_W::new(self, 2)
    }
    #[doc = "Bit 3 - CMD CRC error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn cmde(&mut self) -> CMDE_W<IMR_SPEC> {
        CMDE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Multi Block done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn mbd(&mut self) -> MBD_W<IMR_SPEC> {
        MBD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Multi Block Timeout interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn mbt(&mut self) -> MBT_W<IMR_SPEC> {
        MBT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Cmd and Resp Ncr Timeout interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn crnt(&mut self) -> CRNT_W<IMR_SPEC> {
        CRNT_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC status token err interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<IMR_SPEC> {
        CRCE_W::new(self, 7)
    }
    #[doc = "Bit 8 - SDIO data1 line interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn d1l(&mut self) -> D1L_W<IMR_SPEC> {
        D1L_W::new(self, 8)
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
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
