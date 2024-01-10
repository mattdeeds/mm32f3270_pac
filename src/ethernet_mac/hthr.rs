#[doc = "Register `HTHR` reader"]
pub type R = crate::R<HTHR_SPEC>;
#[doc = "Register `HTHR` writer"]
pub type W = crate::W<HTHR_SPEC>;
#[doc = "Field `HTABH` reader - Hash Table High"]
pub type HTABH_R = crate::FieldReader<u32>;
#[doc = "Field `HTABH` writer - Hash Table High"]
pub type HTABH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    pub fn htabh(&self) -> HTABH_R {
        HTABH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    #[must_use]
    pub fn htabh(&mut self) -> HTABH_W<HTHR_SPEC> {
        HTABH_W::new(self, 0)
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
#[doc = "Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hthr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hthr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTHR_SPEC;
impl crate::RegisterSpec for HTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hthr::R`](R) reader structure"]
impl crate::Readable for HTHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hthr::W`](W) writer structure"]
impl crate::Writable for HTHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTHR to value 0"]
impl crate::Resettable for HTHR_SPEC {
    const RESET_VALUE: u32 = 0;
}
