#[doc = "Register `DMASR` reader"]
pub type R = crate::R<DMASR_SPEC>;
#[doc = "Register `DMASR` writer"]
pub type W = crate::W<DMASR_SPEC>;
#[doc = "Field `TIS` reader - Transmit Interrupt Status"]
pub type TIS_R = crate::BitReader;
#[doc = "Field `TIS` writer - Transmit Interrupt Status"]
pub type TIS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TSS` reader - Transmit Process Stopped Status"]
pub type TSS_R = crate::BitReader;
#[doc = "Field `TSS` writer - Transmit Process Stopped Status"]
pub type TSS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TUS` reader - Transmit Buffer Unavailable Status"]
pub type TUS_R = crate::BitReader;
#[doc = "Field `TUS` writer - Transmit Buffer Unavailable Status"]
pub type TUS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TJS` reader - Transmit Jabber Timeout Status"]
pub type TJS_R = crate::BitReader;
#[doc = "Field `TJS` writer - Transmit Jabber Timeout Status"]
pub type TJS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVS` reader - Receive Overflow Status"]
pub type OVS_R = crate::BitReader;
#[doc = "Field `OVS` writer - Receive Overflow Status"]
pub type OVS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UNS` reader - Transmit Underflow Status"]
pub type UNS_R = crate::BitReader;
#[doc = "Field `UNS` writer - Transmit Underflow Status"]
pub type UNS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RIS` reader - Receive Status"]
pub type RIS_R = crate::BitReader;
#[doc = "Field `RIS` writer - Receive Status"]
pub type RIS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RUS` reader - Receive Buffer Unavailable Status"]
pub type RUS_R = crate::BitReader;
#[doc = "Field `RUS` writer - Receive Buffer Unavailable Status"]
pub type RUS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RSS` reader - Receive Process Stopped Status"]
pub type RSS_R = crate::BitReader;
#[doc = "Field `RSS` writer - Receive Process Stopped Status"]
pub type RSS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RWS` reader - Receive Watchdog Timeout Status"]
pub type RWS_R = crate::BitReader;
#[doc = "Field `RWS` writer - Receive Watchdog Timeout Status"]
pub type RWS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ETS` reader - Early Transmit Interrupt Status"]
pub type ETS_R = crate::BitReader;
#[doc = "Field `ETS` writer - Early Transmit Interrupt Status"]
pub type ETS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FBS` reader - Fatal Bus Error Interrupt Status"]
pub type FBS_R = crate::BitReader;
#[doc = "Field `FBS` writer - Fatal Bus Error Interrupt Status"]
pub type FBS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERS` reader - Early Receive Interrupt Status"]
pub type ERS_R = crate::BitReader;
#[doc = "Field `ERS` writer - Early Receive Interrupt Status"]
pub type ERS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AIS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NIS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RPS` reader - Receive Process State"]
pub type RPS_R = crate::FieldReader;
#[doc = "Field `TPS` reader - Transmit Process State"]
pub type TPS_R = crate::FieldReader;
#[doc = "Field `EBUS` reader - Error Bits Status"]
pub type EBUS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Status"]
    #[inline(always)]
    pub fn tis(&self) -> TIS_R {
        TIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped Status"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Status"]
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Status"]
    #[inline(always)]
    pub fn tjs(&self) -> TJS_R {
        TJS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow Status"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow Status"]
    #[inline(always)]
    pub fn uns(&self) -> UNS_R {
        UNS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Status"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Status"]
    #[inline(always)]
    pub fn rus(&self) -> RUS_R {
        RUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped Status"]
    #[inline(always)]
    pub fn rss(&self) -> RSS_R {
        RSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Status"]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Status"]
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt Status"]
    #[inline(always)]
    pub fn fbs(&self) -> FBS_R {
        FBS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Status"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive Process State"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits Status"]
    #[inline(always)]
    pub fn ebus(&self) -> EBUS_R {
        EBUS_R::new(((self.bits >> 23) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn tis(&mut self) -> TIS_W<DMASR_SPEC> {
        TIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped Status"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<DMASR_SPEC> {
        TSS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Status"]
    #[inline(always)]
    #[must_use]
    pub fn tus(&mut self) -> TUS_W<DMASR_SPEC> {
        TUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Status"]
    #[inline(always)]
    #[must_use]
    pub fn tjs(&mut self) -> TJS_W<DMASR_SPEC> {
        TJS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Overflow Status"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<DMASR_SPEC> {
        OVS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Underflow Status"]
    #[inline(always)]
    #[must_use]
    pub fn uns(&mut self) -> UNS_W<DMASR_SPEC> {
        UNS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Status"]
    #[inline(always)]
    #[must_use]
    pub fn ris(&mut self) -> RIS_W<DMASR_SPEC> {
        RIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Status"]
    #[inline(always)]
    #[must_use]
    pub fn rus(&mut self) -> RUS_W<DMASR_SPEC> {
        RUS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped Status"]
    #[inline(always)]
    #[must_use]
    pub fn rss(&mut self) -> RSS_W<DMASR_SPEC> {
        RSS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Status"]
    #[inline(always)]
    #[must_use]
    pub fn rws(&mut self) -> RWS_W<DMASR_SPEC> {
        RWS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ets(&mut self) -> ETS_W<DMASR_SPEC> {
        ETS_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn fbs(&mut self) -> FBS_W<DMASR_SPEC> {
        FBS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ers(&mut self) -> ERS_W<DMASR_SPEC> {
        ERS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<DMASR_SPEC> {
        AIS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<DMASR_SPEC> {
        NIS_W::new(self, 16)
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
#[doc = "Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASR_SPEC;
impl crate::RegisterSpec for DMASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasr::R`](R) reader structure"]
impl crate::Readable for DMASR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmasr::W`](W) writer structure"]
impl crate::Writable for DMASR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_e7ff;
}
#[doc = "`reset()` method sets DMASR to value 0"]
impl crate::Resettable for DMASR_SPEC {
    const RESET_VALUE: u32 = 0;
}
