#[doc = "Register `DR20` reader"]
pub type R = crate::R<DR20_SPEC>;
#[doc = "Register `DR20` writer"]
pub type W = crate::W<DR20_SPEC>;
#[doc = "Field `BKP` reader - Backup data"]
pub type BKP_R = crate::FieldReader<u16>;
#[doc = "Field `BKP` writer - Backup data"]
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<DR20_SPEC> {
        BKP_W::new(self, 0)
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
#[doc = "Backup data register(BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR20_SPEC;
impl crate::RegisterSpec for DR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr20::R`](R) reader structure"]
impl crate::Readable for DR20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr20::W`](W) writer structure"]
impl crate::Writable for DR20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR20 to value 0"]
impl crate::Resettable for DR20_SPEC {
    const RESET_VALUE: u32 = 0;
}
