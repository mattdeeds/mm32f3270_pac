#[doc = "Register `CIR` reader"]
pub type R = crate::R<CIR_SPEC>;
#[doc = "Register `CIR` writer"]
pub type W = crate::W<CIR_SPEC>;
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader;
#[doc = "Field `LSIRDYF` writer - LSI ready interrupt flag"]
pub type LSIRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYF` reader - *D1"]
pub type LSERDYF_R = crate::BitReader;
#[doc = "Field `LSERDYF` writer - *D1"]
pub type LSERDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader;
#[doc = "Field `HSIRDYF` writer - HSI ready interrupt flag"]
pub type HSIRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader;
#[doc = "Field `HSERDYF` writer - HSE ready interrupt flag"]
pub type HSERDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYF` reader - *D4"]
pub type PLLRDYF_R = crate::BitReader;
#[doc = "Field `PLLRDYF` writer - *D4"]
pub type PLLRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader;
#[doc = "Field `CSSF` writer - Clock security system interrupt flag"]
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LSIRDYIE_R = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYFIE` reader - *D9"]
pub type LSERDYFIE_R = crate::BitReader;
#[doc = "Field `LSERDYFIE` writer - *D9"]
pub type LSERDYFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HSIRDYIE_R = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HSERDYIE_R = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYIE` reader - *D12"]
pub type PLLRDYIE_R = crate::BitReader;
#[doc = "Field `PLLRDYIE` writer - *D12"]
pub type PLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYC` reader - LSI ready interrupt clear"]
pub type LSIRDYC_R = crate::BitReader;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LSIRDYCF` reader - *D17"]
pub type LSIRDYCF_R = crate::BitReader;
#[doc = "Field `LSIRDYCF` writer - *D17"]
pub type LSIRDYCF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HSIRDYC` reader - HSI ready interrupt clear"]
pub type HSIRDYC_R = crate::BitReader;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HSIRDYC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HSERDYC` reader - HSE ready interrupt clear"]
pub type HSERDYC_R = crate::BitReader;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HSERDYC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PLLRDYC` reader - *D20"]
pub type PLLRDYC_R = crate::BitReader;
#[doc = "Field `PLLRDYC` writer - *D20"]
pub type PLLRDYC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CSSC` reader - Clock security system interrupt clear"]
pub type CSSC_R = crate::BitReader;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - *D1"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - *D4"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - *D9"]
    #[inline(always)]
    pub fn lserdyfie(&self) -> LSERDYFIE_R {
        LSERDYFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - *D12"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - *D17"]
    #[inline(always)]
    pub fn lsirdycf(&self) -> LSIRDYCF_R {
        LSIRDYCF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&self) -> HSIRDYC_R {
        HSIRDYC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - *D20"]
    #[inline(always)]
    pub fn pllrdyc(&self) -> PLLRDYC_R {
        PLLRDYC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&self) -> CSSC_R {
        CSSC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W<CIR_SPEC> {
        LSIRDYF_W::new(self, 0)
    }
    #[doc = "Bit 1 - *D1"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyf(&mut self) -> LSERDYF_W<CIR_SPEC> {
        LSERDYF_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W<CIR_SPEC> {
        HSIRDYF_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyf(&mut self) -> HSERDYF_W<CIR_SPEC> {
        HSERDYF_W::new(self, 3)
    }
    #[doc = "Bit 4 - *D4"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyf(&mut self) -> PLLRDYF_W<CIR_SPEC> {
        PLLRDYF_W::new(self, 4)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<CIR_SPEC> {
        CSSF_W::new(self, 7)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIR_SPEC> {
        LSIRDYIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - *D9"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyfie(&mut self) -> LSERDYFIE_W<CIR_SPEC> {
        LSERDYFIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIR_SPEC> {
        HSIRDYIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIR_SPEC> {
        HSERDYIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - *D12"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<CIR_SPEC> {
        PLLRDYIE_W::new(self, 12)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CIR_SPEC> {
        LSIRDYC_W::new(self, 16)
    }
    #[doc = "Bit 17 - *D17"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdycf(&mut self) -> LSIRDYCF_W<CIR_SPEC> {
        LSIRDYCF_W::new(self, 17)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CIR_SPEC> {
        HSIRDYC_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CIR_SPEC> {
        HSERDYC_W::new(self, 19)
    }
    #[doc = "Bit 20 - *D20"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<CIR_SPEC> {
        PLLRDYC_W::new(self, 20)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<CIR_SPEC> {
        CSSC_W::new(self, 23)
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
#[doc = "Clock Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_SPEC;
impl crate::RegisterSpec for CIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir::R`](R) reader structure"]
impl crate::Readable for CIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir::W`](W) writer structure"]
impl crate::Writable for CIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x009f_0000;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIR_SPEC {
    const RESET_VALUE: u32 = 0;
}
