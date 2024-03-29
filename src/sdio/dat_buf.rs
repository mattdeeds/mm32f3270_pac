#[doc = "Register `DAT_BUF` reader"]
pub type R = crate::R<DAT_BUF_SPEC>;
#[doc = "Register `DAT_BUF` writer"]
pub type W = crate::W<DAT_BUF_SPEC>;
#[doc = "Field `DAT` reader - Data buffer"]
pub type DAT_R = crate::FieldReader<u32>;
#[doc = "Field `DAT` writer - Data buffer"]
pub type DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data buffer"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<DAT_BUF_SPEC> {
        DAT_W::new(self, 0)
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
#[doc = "Data buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat_buf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat_buf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAT_BUF_SPEC;
impl crate::RegisterSpec for DAT_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat_buf::R`](R) reader structure"]
impl crate::Readable for DAT_BUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dat_buf::W`](W) writer structure"]
impl crate::Writable for DAT_BUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAT_BUF to value 0"]
impl crate::Resettable for DAT_BUF_SPEC {
    const RESET_VALUE: u32 = 0;
}
