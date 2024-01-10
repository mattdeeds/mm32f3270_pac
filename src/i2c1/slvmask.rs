#[doc = "Register `SLVMASK` reader"]
pub type R = crate::R<SLVMASK_SPEC>;
#[doc = "Register `SLVMASK` writer"]
pub type W = crate::W<SLVMASK_SPEC>;
#[doc = "Field `Mask` reader - Slave Address Mask"]
pub type MASK_R = crate::FieldReader<u16>;
#[doc = "Field `Mask` writer - Slave Address Mask"]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<SLVMASK_SPEC> {
        MASK_W::new(self, 0)
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
#[doc = "Slave Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slvmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slvmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLVMASK_SPEC;
impl crate::RegisterSpec for SLVMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvmask::R`](R) reader structure"]
impl crate::Readable for SLVMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slvmask::W`](W) writer structure"]
impl crate::Writable for SLVMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLVMASK to value 0x03ff"]
impl crate::Resettable for SLVMASK_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
