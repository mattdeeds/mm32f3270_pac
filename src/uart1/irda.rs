#[doc = "Register `IRDA` reader"]
pub type R = crate::R<IRDA_SPEC>;
#[doc = "Register `IRDA` writer"]
pub type W = crate::W<IRDA_SPEC>;
#[doc = "Field `SIREN` reader - IrDA mode enable"]
pub type SIREN_R = crate::BitReader;
#[doc = "Field `SIREN` writer - IrDA mode enable"]
pub type SIREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRLP` reader - IrDA low-power"]
pub type SIRLP_R = crate::BitReader;
#[doc = "Field `SIRLP` writer - IrDA low-power"]
pub type SIRLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSC_REG` reader - *D4"]
pub type PSC_REG_R = crate::FieldReader;
#[doc = "Field `PSC_REG` writer - *D4"]
pub type PSC_REG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - IrDA mode enable"]
    #[inline(always)]
    pub fn siren(&self) -> SIREN_R {
        SIREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA low-power"]
    #[inline(always)]
    pub fn sirlp(&self) -> SIRLP_R {
        SIRLP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - *D4"]
    #[inline(always)]
    pub fn psc_reg(&self) -> PSC_REG_R {
        PSC_REG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn siren(&mut self) -> SIREN_W<IRDA_SPEC> {
        SIREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn sirlp(&mut self) -> SIRLP_W<IRDA_SPEC> {
        SIRLP_W::new(self, 1)
    }
    #[doc = "Bits 8:15 - *D4"]
    #[inline(always)]
    #[must_use]
    pub fn psc_reg(&mut self) -> PSC_REG_W<IRDA_SPEC> {
        PSC_REG_W::new(self, 8)
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
#[doc = "IrDA configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRDA_SPEC;
impl crate::RegisterSpec for IRDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irda::R`](R) reader structure"]
impl crate::Readable for IRDA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irda::W`](W) writer structure"]
impl crate::Writable for IRDA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRDA to value 0x0100"]
impl crate::Resettable for IRDA_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
