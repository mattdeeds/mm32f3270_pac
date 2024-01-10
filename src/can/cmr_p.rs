#[doc = "Register `CMR_P` writer"]
pub type W = crate::W<CMR_P_SPEC>;
#[doc = "Field `TR` writer - Transmission request"]
pub type TR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AT` writer - Abort transmission"]
pub type AT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRB` writer - Release receive buffer"]
pub type RRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDO` writer - Clear data overrun"]
pub type CDO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRR` writer - Self reset request"]
pub type SRR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERB` writer - Empty Rxbuffer"]
pub type ERB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission request"]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TR_W<CMR_P_SPEC> {
        TR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Abort transmission"]
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AT_W<CMR_P_SPEC> {
        AT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Release receive buffer"]
    #[inline(always)]
    #[must_use]
    pub fn rrb(&mut self) -> RRB_W<CMR_P_SPEC> {
        RRB_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear data overrun"]
    #[inline(always)]
    #[must_use]
    pub fn cdo(&mut self) -> CDO_W<CMR_P_SPEC> {
        CDO_W::new(self, 3)
    }
    #[doc = "Bit 4 - Self reset request"]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SRR_W<CMR_P_SPEC> {
        SRR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Empty Rxbuffer"]
    #[inline(always)]
    #[must_use]
    pub fn erb(&mut self) -> ERB_W<CMR_P_SPEC> {
        ERB_W::new(self, 5)
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
#[doc = "Peli Command register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr_p::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMR_P_SPEC;
impl crate::RegisterSpec for CMR_P_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmr_p::W`](W) writer structure"]
impl crate::Writable for CMR_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR_P to value 0"]
impl crate::Resettable for CMR_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
