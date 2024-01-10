#[doc = "Register `SETUP` reader"]
pub type R = crate::R<SETUP_SPEC>;
#[doc = "Register `SETUP` writer"]
pub type W = crate::W<SETUP_SPEC>;
#[doc = "Field `CNT` reader - SDA setup"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - SDA setup"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SDA setup"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDA setup"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<SETUP_SPEC> {
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
#[doc = "SDA Setup Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SETUP_SPEC;
impl crate::RegisterSpec for SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setup::R`](R) reader structure"]
impl crate::Readable for SETUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`setup::W`](W) writer structure"]
impl crate::Writable for SETUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETUP to value 0x64"]
impl crate::Resettable for SETUP_SPEC {
    const RESET_VALUE: u32 = 0x64;
}
