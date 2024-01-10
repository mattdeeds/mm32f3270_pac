#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCR_SPEC>;
#[doc = "Field `CGIF1` writer - channel 1 global interrupt clear"]
pub type CGIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - channel 1 transfer complete clear"]
pub type CTCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - channel 1 half transfer clear"]
pub type CHTIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - channel 1 transfer error clear"]
pub type CTEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF2` writer - channel 2 global interrupt clear"]
pub type CGIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - channel 2 transfer complete clear"]
pub type CTCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - channel 2 half transfer clear"]
pub type CHTIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - channel 2 transfer error clear"]
pub type CTEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF3` writer - channel 3 global interrupt clear"]
pub type CGIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - channel 3 transfer complete clear"]
pub type CTCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - channel 3 half transfer clear"]
pub type CHTIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - channel 3 transfer error clear"]
pub type CTEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF4` writer - channel 4 global interrupt clear"]
pub type CGIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` writer - channel 4 transfer complete clear"]
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` writer - channel 4 half transfer clear"]
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` writer - channel 4 transfer error clear"]
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF5` writer - channel 5 global interrupt clear"]
pub type CGIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` writer - channel 5 transfer complete clear"]
pub type CTCIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` writer - channel 5 half transfer clear"]
pub type CHTIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` writer - channel 5 transfer error clear"]
pub type CTEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF6` writer - channel 6 global interrupt clear"]
pub type CGIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF6` writer - channel 6 transfer complete clear"]
pub type CTCIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF6` writer - channel 6 half transfer clear"]
pub type CHTIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF6` writer - channel 6 transfer error clear"]
pub type CTEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF7` writer - channel 7 global interrupt clear"]
pub type CGIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF7` writer - channel 7 transfer complete clear"]
pub type CTCIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF7` writer - channel 7 half transfer clear"]
pub type CHTIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF7` writer - channel 7 transfer error clear"]
pub type CTEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - channel 1 global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF1_W<IFCR_SPEC> {
        CGIF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - channel 1 transfer complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<IFCR_SPEC> {
        CTCIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - channel 1 half transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<IFCR_SPEC> {
        CHTIF1_W::new(self, 2)
    }
    #[doc = "Bit 3 - channel 1 transfer error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<IFCR_SPEC> {
        CTEIF1_W::new(self, 3)
    }
    #[doc = "Bit 4 - channel 2 global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF2_W<IFCR_SPEC> {
        CGIF2_W::new(self, 4)
    }
    #[doc = "Bit 5 - channel 2 transfer complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<IFCR_SPEC> {
        CTCIF2_W::new(self, 5)
    }
    #[doc = "Bit 6 - channel 2 half transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<IFCR_SPEC> {
        CHTIF2_W::new(self, 6)
    }
    #[doc = "Bit 7 - channel 2 transfer error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<IFCR_SPEC> {
        CTEIF2_W::new(self, 7)
    }
    #[doc = "Bit 8 - channel 3 global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF3_W<IFCR_SPEC> {
        CGIF3_W::new(self, 8)
    }
    #[doc = "Bit 9 - channel 3 transfer complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<IFCR_SPEC> {
        CTCIF3_W::new(self, 9)
    }
    #[doc = "Bit 10 - channel 3 half transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<IFCR_SPEC> {
        CHTIF3_W::new(self, 10)
    }
    #[doc = "Bit 11 - channel 3 transfer error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<IFCR_SPEC> {
        CTEIF3_W::new(self, 11)
    }
    #[doc = "Bit 12 - channel 4 global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif4(&mut self) -> CGIF4_W<IFCR_SPEC> {
        CGIF4_W::new(self, 12)
    }
    #[doc = "Bit 13 - channel 4 transfer complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF4_W<IFCR_SPEC> {
        CTCIF4_W::new(self, 13)
    }
    #[doc = "Bit 14 - channel 4 half transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF4_W<IFCR_SPEC> {
        CHTIF4_W::new(self, 14)
    }
    #[doc = "Bit 15 - channel 4 transfer error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF4_W<IFCR_SPEC> {
        CTEIF4_W::new(self, 15)
    }
    #[doc = "Bit 16 - channel 5 global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif5(&mut self) -> CGIF5_W<IFCR_SPEC> {
        CGIF5_W::new(self, 16)
    }
    #[doc = "Bit 17 - channel 5 transfer complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF5_W<IFCR_SPEC> {
        CTCIF5_W::new(self, 17)
    }
    #[doc = "Bit 18 - channel 5 half transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF5_W<IFCR_SPEC> {
        CHTIF5_W::new(self, 18)
    }
    #[doc = "Bit 19 - channel 5 transfer error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF5_W<IFCR_SPEC> {
        CTEIF5_W::new(self, 19)
    }
    #[doc = "Bit 20 - channel 6 global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif6(&mut self) -> CGIF6_W<IFCR_SPEC> {
        CGIF6_W::new(self, 20)
    }
    #[doc = "Bit 21 - channel 6 transfer complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> CTCIF6_W<IFCR_SPEC> {
        CTCIF6_W::new(self, 21)
    }
    #[doc = "Bit 22 - channel 6 half transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> CHTIF6_W<IFCR_SPEC> {
        CHTIF6_W::new(self, 22)
    }
    #[doc = "Bit 23 - channel 6 transfer error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF6_W<IFCR_SPEC> {
        CTEIF6_W::new(self, 23)
    }
    #[doc = "Bit 24 - channel 7 global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif7(&mut self) -> CGIF7_W<IFCR_SPEC> {
        CGIF7_W::new(self, 24)
    }
    #[doc = "Bit 25 - channel 7 transfer complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> CTCIF7_W<IFCR_SPEC> {
        CTCIF7_W::new(self, 25)
    }
    #[doc = "Bit 26 - channel 7 half transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> CHTIF7_W<IFCR_SPEC> {
        CHTIF7_W::new(self, 26)
    }
    #[doc = "Bit 27 - channel 7 transfer error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> CTEIF7_W<IFCR_SPEC> {
        CTEIF7_W::new(self, 27)
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
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
