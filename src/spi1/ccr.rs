#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `CPHA` reader - Clock phase select bit"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase select bit"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity select bit"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity select bit"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFE` reader - LSI first enable bit"]
pub type LSBFE_R = crate::BitReader;
#[doc = "Field `LSBFE` writer - LSI first enable bit"]
pub type LSBFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPILEN` reader - SPI character length bit"]
pub type SPILEN_R = crate::BitReader;
#[doc = "Field `SPILEN` writer - SPI character length bit"]
pub type SPILEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEDGE` reader - Receive data edge select"]
pub type RXEDGE_R = crate::BitReader;
#[doc = "Field `RXEDGE` writer - Receive data edge select"]
pub type RXEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEDGE` reader - Transmit data edge select"]
pub type TXEDGE_R = crate::BitReader;
#[doc = "Field `TXEDGE` writer - Transmit data edge select"]
pub type TXEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHASEL` reader - CPHA polarity select"]
pub type CPHASEL_R = crate::BitReader;
#[doc = "Field `CPHASEL` writer - CPHA polarity select"]
pub type CPHASEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock phase select bit"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity select bit"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSI first enable bit"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LSBFE_R {
        LSBFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI character length bit"]
    #[inline(always)]
    pub fn spilen(&self) -> SPILEN_R {
        SPILEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive data edge select"]
    #[inline(always)]
    pub fn rxedge(&self) -> RXEDGE_R {
        RXEDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit data edge select"]
    #[inline(always)]
    pub fn txedge(&self) -> TXEDGE_R {
        TXEDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPHA polarity select"]
    #[inline(always)]
    pub fn cphasel(&self) -> CPHASEL_R {
        CPHASEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase select bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CCR_SPEC> {
        CPHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity select bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CCR_SPEC> {
        CPOL_W::new(self, 1)
    }
    #[doc = "Bit 2 - LSI first enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfe(&mut self) -> LSBFE_W<CCR_SPEC> {
        LSBFE_W::new(self, 2)
    }
    #[doc = "Bit 3 - SPI character length bit"]
    #[inline(always)]
    #[must_use]
    pub fn spilen(&mut self) -> SPILEN_W<CCR_SPEC> {
        SPILEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive data edge select"]
    #[inline(always)]
    #[must_use]
    pub fn rxedge(&mut self) -> RXEDGE_W<CCR_SPEC> {
        RXEDGE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit data edge select"]
    #[inline(always)]
    #[must_use]
    pub fn txedge(&mut self) -> TXEDGE_W<CCR_SPEC> {
        TXEDGE_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPHA polarity select"]
    #[inline(always)]
    #[must_use]
    pub fn cphasel(&mut self) -> CPHASEL_W<CCR_SPEC> {
        CPHASEL_W::new(self, 6)
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
#[doc = "Current control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0x08"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
