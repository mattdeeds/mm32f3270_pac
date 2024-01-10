#[doc = "Register `NSSR` reader"]
pub type R = crate::R<NSSR_SPEC>;
#[doc = "Register `NSSR` writer"]
pub type W = crate::W<NSSR_SPEC>;
#[doc = "Field `NSS` reader - Chip select output signal in Master mode"]
pub type NSS_R = crate::FieldReader;
#[doc = "Field `NSS` writer - Chip select output signal in Master mode"]
pub type NSS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Chip select output signal in Master mode"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Chip select output signal in Master mode"]
    #[inline(always)]
    #[must_use]
    pub fn nss(&mut self) -> NSS_W<NSSR_SPEC> {
        NSS_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Slave chip select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSSR_SPEC;
impl crate::RegisterSpec for NSSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nssr::R`](R) reader structure"]
impl crate::Readable for NSSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nssr::W`](W) writer structure"]
impl crate::Writable for NSSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NSSR to value 0xff"]
impl crate::Resettable for NSSR_SPEC {
    const RESET_VALUE: u16 = 0xff;
}
