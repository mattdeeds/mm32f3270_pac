#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCR_SPEC>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "Field `FCBBPA` reader - Flow Control Busy/Back Pressure Activate"]
pub type FCBBPA_R = crate::BitReader;
#[doc = "Field `FCBBPA` writer - Flow Control Busy/Back Pressure Activate"]
pub type FCBBPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTE` reader - Transmit Flow Control Enable"]
pub type FTE_R = crate::BitReader;
#[doc = "Field `FTE` writer - Transmit Flow Control Enable"]
pub type FTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRE` reader - Receive Flow Control Enable"]
pub type FRE_R = crate::BitReader;
#[doc = "Field `FRE` writer - Receive Flow Control Enable"]
pub type FRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect"]
pub type UP_R = crate::BitReader;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect"]
pub type UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DZPQ` reader - When the flow control signal is valid, it is forbidden to generate a zero-slice pause control frame"]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - When the flow control signal is valid, it is forbidden to generate a zero-slice pause control frame"]
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSET` reader - Pause time"]
pub type PSET_R = crate::FieldReader<u16>;
#[doc = "Field `PSET` writer - Pause time"]
pub type PSET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy/Back Pressure Activate"]
    #[inline(always)]
    pub fn fcbbpa(&self) -> FCBBPA_R {
        FCBBPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn fte(&self) -> FTE_R {
        FTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - When the flow control signal is valid, it is forbidden to generate a zero-slice pause control frame"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn pset(&self) -> PSET_R {
        PSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy/Back Pressure Activate"]
    #[inline(always)]
    #[must_use]
    pub fn fcbbpa(&mut self) -> FCBBPA_W<FCR_SPEC> {
        FCBBPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fte(&mut self) -> FTE_W<FCR_SPEC> {
        FTE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fre(&mut self) -> FRE_W<FCR_SPEC> {
        FRE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UP_W<FCR_SPEC> {
        UP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<FCR_SPEC> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - When the flow control signal is valid, it is forbidden to generate a zero-slice pause control frame"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<FCR_SPEC> {
        DZPQ_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    #[must_use]
    pub fn pset(&mut self) -> PSET_W<FCR_SPEC> {
        PSET_W::new(self, 16)
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
#[doc = "Ethernet MAC flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
