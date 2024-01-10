#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `RBS` reader - Receive buffer status"]
pub type RBS_R = crate::BitReader;
#[doc = "Field `RBS` writer - Receive buffer status"]
pub type RBS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOS` reader - Data overrun status"]
pub type DOS_R = crate::BitReader;
#[doc = "Field `DOS` writer - Data overrun status"]
pub type DOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBS` reader - Transmit buffer status"]
pub type TBS_R = crate::BitReader;
#[doc = "Field `TBS` writer - Transmit buffer status"]
pub type TBS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCS` reader - Transmission complete status"]
pub type TCS_R = crate::BitReader;
#[doc = "Field `TCS` writer - Transmission complete status"]
pub type TCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Receive status"]
pub type RS_R = crate::BitReader;
#[doc = "Field `RS` writer - Receive status"]
pub type RS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - Transmit status"]
pub type TS_R = crate::BitReader;
#[doc = "Field `TS` writer - Transmit status"]
pub type TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ES` reader - Error status"]
pub type ES_R = crate::BitReader;
#[doc = "Field `ES` writer - Error status"]
pub type ES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS` reader - Bus status"]
pub type BS_R = crate::BitReader;
#[doc = "Field `BS` writer - Bus status"]
pub type BS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive buffer status"]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data overrun status"]
    #[inline(always)]
    pub fn dos(&self) -> DOS_R {
        DOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer status"]
    #[inline(always)]
    pub fn tbs(&self) -> TBS_R {
        TBS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission complete status"]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive status"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit status"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error status"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus status"]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive buffer status"]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<SR_SPEC> {
        RBS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data overrun status"]
    #[inline(always)]
    #[must_use]
    pub fn dos(&mut self) -> DOS_W<SR_SPEC> {
        DOS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer status"]
    #[inline(always)]
    #[must_use]
    pub fn tbs(&mut self) -> TBS_W<SR_SPEC> {
        TBS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmission complete status"]
    #[inline(always)]
    #[must_use]
    pub fn tcs(&mut self) -> TCS_W<SR_SPEC> {
        TCS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive status"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<SR_SPEC> {
        RS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit status"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<SR_SPEC> {
        TS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Error status"]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> ES_W<SR_SPEC> {
        ES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bus status"]
    #[inline(always)]
    #[must_use]
    pub fn bs(&mut self) -> BS_W<SR_SPEC> {
        BS_W::new(self, 7)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x0c"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
