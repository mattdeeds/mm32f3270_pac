#[doc = "Register `RDNR` reader"]
pub type R = crate::R<RDNR_SPEC>;
#[doc = "Register `RDNR` writer"]
pub type W = crate::W<RDNR_SPEC>;
#[doc = "Field `RDN` reader - The register is used to hold a count of to be received bytes in next receive process"]
pub type RDN_R = crate::FieldReader<u16>;
#[doc = "Field `RDN` writer - The register is used to hold a count of to be received bytes in next receive process"]
pub type RDN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    pub fn rdn(&self) -> RDN_R {
        RDN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    #[must_use]
    pub fn rdn(&mut self) -> RDN_W<RDNR_SPEC> {
        RDN_W::new(self, 0)
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
#[doc = "Receive data count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDNR_SPEC;
impl crate::RegisterSpec for RDNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdnr::R`](R) reader structure"]
impl crate::Readable for RDNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdnr::W`](W) writer structure"]
impl crate::Writable for RDNR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDNR to value 0x01"]
impl crate::Resettable for RDNR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
