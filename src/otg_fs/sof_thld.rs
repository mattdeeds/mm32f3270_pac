#[doc = "Register `SOF_THLD` reader"]
pub type R = crate::R<SOF_THLD_SPEC>;
#[doc = "Register `SOF_THLD` writer"]
pub type W = crate::W<SOF_THLD_SPEC>;
#[doc = "Field `CNT` reader - These bits represent the sof count threshold for byte time"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - These bits represent the sof count threshold for byte time"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits represent the sof count threshold for byte time"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits represent the sof count threshold for byte time"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<SOF_THLD_SPEC> {
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
#[doc = "SOF threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sof_thld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sof_thld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF_THLD_SPEC;
impl crate::RegisterSpec for SOF_THLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof_thld::R`](R) reader structure"]
impl crate::Readable for SOF_THLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sof_thld::W`](W) writer structure"]
impl crate::Writable for SOF_THLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOF_THLD to value 0"]
impl crate::Resettable for SOF_THLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
