#[doc = "Register `DHR12LD` reader"]
pub type R = crate::R<DHR12LD_SPEC>;
#[doc = "Register `DHR12LD` writer"]
pub type W = crate::W<DHR12LD_SPEC>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit left-aligned data"]
pub type DACC1DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit left-aligned data"]
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit left-aligned data"]
pub type DACC2DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit left-aligned data"]
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DAC channel2 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<DHR12LD_SPEC> {
        DACC1DHR_W::new(self, 4)
    }
    #[doc = "Bits 20:31 - DAC channel2 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<DHR12LD_SPEC> {
        DACC2DHR_W::new(self, 20)
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
#[doc = "Dual channel 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr12ld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr12ld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR12LD_SPEC;
impl crate::RegisterSpec for DHR12LD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12ld::R`](R) reader structure"]
impl crate::Readable for DHR12LD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dhr12ld::W`](W) writer structure"]
impl crate::Writable for DHR12LD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHR12LD to value 0"]
impl crate::Resettable for DHR12LD_SPEC {
    const RESET_VALUE: u32 = 0;
}
