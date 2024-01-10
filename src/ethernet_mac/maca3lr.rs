#[doc = "Register `MACA3LR` reader"]
pub type R = crate::R<MACA3LR_SPEC>;
#[doc = "Register `MACA3LR` writer"]
pub type W = crate::W<MACA3LR_SPEC>;
#[doc = "Field `ADDL` reader - MAC address3 low \\[31:0\\]"]
pub type ADDL_R = crate::FieldReader<u32>;
#[doc = "Field `ADDL` writer - MAC address3 low \\[31:0\\]"]
pub type ADDL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address3 low \\[31:0\\]"]
    #[inline(always)]
    pub fn addl(&self) -> ADDL_R {
        ADDL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address3 low \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addl(&mut self) -> ADDL_W<MACA3LR_SPEC> {
        ADDL_W::new(self, 0)
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
#[doc = "Ethernet MAC address3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA3LR_SPEC;
impl crate::RegisterSpec for MACA3LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3lr::R`](R) reader structure"]
impl crate::Readable for MACA3LR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca3lr::W`](W) writer structure"]
impl crate::Writable for MACA3LR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA3LR to value 0"]
impl crate::Resettable for MACA3LR_SPEC {
    const RESET_VALUE: u32 = 0;
}
