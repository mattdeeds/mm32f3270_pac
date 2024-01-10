#[doc = "Register `BUFCR` reader"]
pub type R = crate::R<BUFCR_SPEC>;
#[doc = "Register `BUFCR` writer"]
pub type W = crate::W<BUFCR_SPEC>;
#[doc = "Field `DBF` reader - Data buff full"]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - Data buff full"]
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBE` reader - Data buff empty. read-only"]
pub type DBE_R = crate::BitReader;
#[doc = "Field `DBE` writer - Data buff empty. read-only"]
pub type DBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBML` reader - Data buff data water mark level"]
pub type DBML_R = crate::FieldReader;
#[doc = "Field `DBML` writer - Data buff data water mark level"]
pub type DBML_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMAHEN` reader - DMA hardware interface enable"]
pub type DMAHEN_R = crate::BitReader;
#[doc = "Field `DMAHEN` writer - DMA hardware interface enable"]
pub type DMAHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BADIR` reader - Set buff access direction"]
pub type BADIR_R = crate::BitReader;
#[doc = "Field `BADIR` writer - Set buff access direction"]
pub type BADIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFIFOSM` reader - Data FIFO status signal mask bit"]
pub type DFIFOSM_R = crate::BitReader;
#[doc = "Field `DFIFOSM` writer - Data FIFO status signal mask bit"]
pub type DFIFOSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARM` reader - DMA Requst mask"]
pub type DMARM_R = crate::BitReader;
#[doc = "Field `DMARM` writer - DMA Requst mask"]
pub type DMARM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBFEN` reader - Data Buf flush enable"]
pub type DBFEN_R = crate::BitReader;
#[doc = "Field `DBFEN` writer - Data Buf flush enable"]
pub type DBFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data buff full"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data buff empty. read-only"]
    #[inline(always)]
    pub fn dbe(&self) -> DBE_R {
        DBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Data buff data water mark level"]
    #[inline(always)]
    pub fn dbml(&self) -> DBML_R {
        DBML_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - DMA hardware interface enable"]
    #[inline(always)]
    pub fn dmahen(&self) -> DMAHEN_R {
        DMAHEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set buff access direction"]
    #[inline(always)]
    pub fn badir(&self) -> BADIR_R {
        BADIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data FIFO status signal mask bit"]
    #[inline(always)]
    pub fn dfifosm(&self) -> DFIFOSM_R {
        DFIFOSM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA Requst mask"]
    #[inline(always)]
    pub fn dmarm(&self) -> DMARM_R {
        DMARM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Buf flush enable"]
    #[inline(always)]
    pub fn dbfen(&self) -> DBFEN_R {
        DBFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data buff full"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<BUFCR_SPEC> {
        DBF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data buff empty. read-only"]
    #[inline(always)]
    #[must_use]
    pub fn dbe(&mut self) -> DBE_W<BUFCR_SPEC> {
        DBE_W::new(self, 1)
    }
    #[doc = "Bits 2:9 - Data buff data water mark level"]
    #[inline(always)]
    #[must_use]
    pub fn dbml(&mut self) -> DBML_W<BUFCR_SPEC> {
        DBML_W::new(self, 2)
    }
    #[doc = "Bit 10 - DMA hardware interface enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmahen(&mut self) -> DMAHEN_W<BUFCR_SPEC> {
        DMAHEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set buff access direction"]
    #[inline(always)]
    #[must_use]
    pub fn badir(&mut self) -> BADIR_W<BUFCR_SPEC> {
        BADIR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Data FIFO status signal mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn dfifosm(&mut self) -> DFIFOSM_W<BUFCR_SPEC> {
        DFIFOSM_W::new(self, 12)
    }
    #[doc = "Bit 14 - DMA Requst mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmarm(&mut self) -> DMARM_W<BUFCR_SPEC> {
        DMARM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Data Buf flush enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbfen(&mut self) -> DBFEN_W<BUFCR_SPEC> {
        DBFEN_W::new(self, 15)
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
#[doc = "Buffer control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFCR_SPEC;
impl crate::RegisterSpec for BUFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufcr::R`](R) reader structure"]
impl crate::Readable for BUFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bufcr::W`](W) writer structure"]
impl crate::Writable for BUFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUFCR to value 0x02"]
impl crate::Resettable for BUFCR_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
