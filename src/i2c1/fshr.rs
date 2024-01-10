#[doc = "Register `FSHR` reader"]
pub type R = crate::R<FSHR_SPEC>;
#[doc = "Register `FSHR` writer"]
pub type W = crate::W<FSHR_SPEC>;
#[doc = "Field `CNT` reader - This register sets the SCL clock high_period count for standard speed"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - This register sets the SCL clock high_period count for standard speed"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register sets the SCL clock high_period count for standard speed"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register sets the SCL clock high_period count for standard speed"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<FSHR_SPEC> {
        CNT_W::new(self, 0)
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
#[doc = "SCL High Period Count for Fast Speed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fshr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fshr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSHR_SPEC;
impl crate::RegisterSpec for FSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fshr::R`](R) reader structure"]
impl crate::Readable for FSHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fshr::W`](W) writer structure"]
impl crate::Writable for FSHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSHR to value 0x3c"]
impl crate::Resettable for FSHR_SPEC {
    const RESET_VALUE: u32 = 0x3c;
}
