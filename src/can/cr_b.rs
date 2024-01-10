#[doc = "Register `CR_B` reader"]
pub type R = crate::R<CR_B_SPEC>;
#[doc = "Register `CR_B` writer"]
pub type W = crate::W<CR_B_SPEC>;
#[doc = "Field `RR` reader - Reset request"]
pub type RR_R = crate::BitReader;
#[doc = "Field `RR` writer - Reset request"]
pub type RR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIE` reader - Overflow interrupt enable"]
pub type OIE_R = crate::BitReader;
#[doc = "Field `OIE` writer - Overflow interrupt enable"]
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset request"]
    #[inline(always)]
    pub fn rr(&self) -> RR_R {
        RR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset request"]
    #[inline(always)]
    #[must_use]
    pub fn rr(&mut self) -> RR_W<CR_B_SPEC> {
        RR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<CR_B_SPEC> {
        RIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<CR_B_SPEC> {
        TIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<CR_B_SPEC> {
        EIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<CR_B_SPEC> {
        OIE_W::new(self, 4)
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
#[doc = "Basic control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_B_SPEC;
impl crate::RegisterSpec for CR_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_b::R`](R) reader structure"]
impl crate::Readable for CR_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr_b::W`](W) writer structure"]
impl crate::Writable for CR_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR_B to value 0x21"]
impl crate::Resettable for CR_B_SPEC {
    const RESET_VALUE: u32 = 0x21;
}
