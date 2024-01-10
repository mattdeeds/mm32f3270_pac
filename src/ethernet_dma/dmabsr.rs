#[doc = "Register `DMABSR` reader"]
pub type R = crate::R<DMABSR_SPEC>;
#[doc = "Register `DMABSR` writer"]
pub type W = crate::W<DMABSR_SPEC>;
#[doc = "Field `SR` reader - Software reset"]
pub type SR_R = crate::BitReader;
#[doc = "Field `SR` writer - Software reset"]
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAA` reader - DMA Arbitration"]
pub type DMAA_R = crate::BitReader;
#[doc = "Field `DMAA` writer - DMA Arbitration"]
pub type DMAA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FTPR` reader - Rx Tx priority ratio"]
pub type FTPR_R = crate::FieldReader;
#[doc = "Field `FTPR` writer - Rx Tx priority ratio"]
pub type FTPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FBST` reader - Fixed burst"]
pub type FBST_R = crate::BitReader;
#[doc = "Field `FBST` writer - Fixed burst"]
pub type FBST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALL` reader - *D25"]
pub type ALL_R = crate::BitReader;
#[doc = "Field `ALL` writer - *D25"]
pub type ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn dmaa(&self) -> DMAA_R {
        DMAA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    pub fn ftpr(&self) -> FTPR_R {
        FTPR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fbst(&self) -> FBST_R {
        FBST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - *D25"]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<DMABSR_SPEC> {
        SR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn dmaa(&mut self) -> DMAA_W<DMABSR_SPEC> {
        DMAA_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<DMABSR_SPEC> {
        DSL_W::new(self, 2)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<DMABSR_SPEC> {
        PBL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ftpr(&mut self) -> FTPR_W<DMABSR_SPEC> {
        FTPR_W::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    #[must_use]
    pub fn fbst(&mut self) -> FBST_W<DMABSR_SPEC> {
        FBST_W::new(self, 16)
    }
    #[doc = "Bit 25 - *D25"]
    #[inline(always)]
    #[must_use]
    pub fn all(&mut self) -> ALL_W<DMABSR_SPEC> {
        ALL_W::new(self, 25)
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
#[doc = "Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMABSR_SPEC;
impl crate::RegisterSpec for DMABSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabsr::R`](R) reader structure"]
impl crate::Readable for DMABSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmabsr::W`](W) writer structure"]
impl crate::Writable for DMABSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMABSR to value 0"]
impl crate::Resettable for DMABSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
