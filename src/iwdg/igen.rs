#[doc = "Register `IGEN` reader"]
pub type R = crate::R<IGEN_SPEC>;
#[doc = "Register `IGEN` writer"]
pub type W = crate::W<IGEN_SPEC>;
#[doc = "Field `IGEN` reader - Watchdog Interrupt Generate value"]
pub type IGEN_R = crate::FieldReader<u16>;
#[doc = "Field `IGEN` writer - Watchdog Interrupt Generate value"]
pub type IGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog Interrupt Generate value"]
    #[inline(always)]
    pub fn igen(&self) -> IGEN_R {
        IGEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog Interrupt Generate value"]
    #[inline(always)]
    #[must_use]
    pub fn igen(&mut self) -> IGEN_W<IGEN_SPEC> {
        IGEN_W::new(self, 0)
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
#[doc = "Interruput generate value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`igen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`igen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IGEN_SPEC;
impl crate::RegisterSpec for IGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`igen::R`](R) reader structure"]
impl crate::Readable for IGEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`igen::W`](W) writer structure"]
impl crate::Writable for IGEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IGEN to value 0x0fff"]
impl crate::Resettable for IGEN_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
