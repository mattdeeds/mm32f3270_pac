#[doc = "Register `BTR0` reader"]
pub type R = crate::R<BTR0_SPEC>;
#[doc = "Register `BTR0` writer"]
pub type W = crate::W<BTR0_SPEC>;
#[doc = "Field `BRP` reader - Baud rate prescaler"]
pub type BRP_R = crate::FieldReader;
#[doc = "Field `BRP` writer - Baud rate prescaler"]
pub type BRP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SJW` reader - Synchronization jump width"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - Synchronization jump width"]
pub type SJW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - Baud rate prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud rate prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<BTR0_SPEC> {
        BRP_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Synchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<BTR0_SPEC> {
        SJW_W::new(self, 6)
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
#[doc = "Bus Timing register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTR0_SPEC;
impl crate::RegisterSpec for BTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr0::R`](R) reader structure"]
impl crate::Readable for BTR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btr0::W`](W) writer structure"]
impl crate::Writable for BTR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTR0 to value 0"]
impl crate::Resettable for BTR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
