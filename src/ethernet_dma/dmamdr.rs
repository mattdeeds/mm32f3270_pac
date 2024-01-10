#[doc = "Register `DMAMDR` reader"]
pub type R = crate::R<DMAMDR_SPEC>;
#[doc = "Register `DMAMDR` writer"]
pub type W = crate::W<DMAMDR_SPEC>;
#[doc = "Field `STR` reader - Start/Stop Receive"]
pub type STR_R = crate::BitReader;
#[doc = "Field `STR` writer - Start/Stop Receive"]
pub type STR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - Receive threshold control"]
pub type RTC_R = crate::FieldReader;
#[doc = "Field `RTC` writer - Receive threshold control"]
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DGF` reader - RxFIFO discards jumbo frames"]
pub type DGF_R = crate::BitReader;
#[doc = "Field `DGF` writer - RxFIFO discards jumbo frames"]
pub type DGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUF` reader - Forward undersized goodframes"]
pub type FUF_R = crate::BitReader;
#[doc = "Field `FUF` writer - Forward undersized goodframes"]
pub type FUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEF` reader - For Error Frames"]
pub type FEF_R = crate::BitReader;
#[doc = "Field `FEF` writer - For Error Frames"]
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STT` reader - Start/stop transmission"]
pub type STT_R = crate::BitReader;
#[doc = "Field `STT` writer - Start/stop transmission"]
pub type STT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTC` reader - Transmit threshold control"]
pub type TTC_R = crate::FieldReader;
#[doc = "Field `TTC` writer - Transmit threshold control"]
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FTF` reader - Flush Transmit FIFO"]
pub type FTF_R = crate::BitReader;
#[doc = "Field `FTF` writer - Flush Transmit FIFO"]
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Transmit Store and Forward"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit Store and Forward"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFRF` reader - When this bit is set to 1, RxDMA will not refresh any frame because the receive descriptor/buffer is unavailable, please refer to the section \"Receiving Process Suspended\""]
pub type DFRF_R = crate::BitReader;
#[doc = "Field `DFRF` writer - When this bit is set to 1, RxDMA will not refresh any frame because the receive descriptor/buffer is unavailable, please refer to the section \"Receiving Process Suspended\""]
pub type DFRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - After the RxFIFO writes a complete frame, it executes the read action while ignoring the RTC bit"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - After the RxFIFO writes a complete frame, it executes the read action while ignoring the RTC bit"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCOE` reader - If there are only errors detected by the receive checksum offload engine in the received frame, the kernel will not discard the frame"]
pub type DTCOE_R = crate::BitReader;
#[doc = "Field `DTCOE` writer - If there are only errors detected by the receive checksum offload engine in the received frame, the kernel will not discard the frame"]
pub type DTCOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start/Stop Receive"]
    #[inline(always)]
    pub fn str(&self) -> STR_R {
        STR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - RxFIFO discards jumbo frames"]
    #[inline(always)]
    pub fn dgf(&self) -> DGF_R {
        DGF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Forward undersized goodframes"]
    #[inline(always)]
    pub fn fuf(&self) -> FUF_R {
        FUF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For Error Frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    pub fn stt(&self) -> STT_R {
        STT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - When this bit is set to 1, RxDMA will not refresh any frame because the receive descriptor/buffer is unavailable, please refer to the section \"Receiving Process Suspended\""]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - After the RxFIFO writes a complete frame, it executes the read action while ignoring the RTC bit"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If there are only errors detected by the receive checksum offload engine in the received frame, the kernel will not discard the frame"]
    #[inline(always)]
    pub fn dtcoe(&self) -> DTCOE_R {
        DTCOE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/Stop Receive"]
    #[inline(always)]
    #[must_use]
    pub fn str(&mut self) -> STR_W<DMAMDR_SPEC> {
        STR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<DMAMDR_SPEC> {
        OSF_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<DMAMDR_SPEC> {
        RTC_W::new(self, 3)
    }
    #[doc = "Bit 5 - RxFIFO discards jumbo frames"]
    #[inline(always)]
    #[must_use]
    pub fn dgf(&mut self) -> DGF_W<DMAMDR_SPEC> {
        DGF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Forward undersized goodframes"]
    #[inline(always)]
    #[must_use]
    pub fn fuf(&mut self) -> FUF_W<DMAMDR_SPEC> {
        FUF_W::new(self, 6)
    }
    #[doc = "Bit 7 - For Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<DMAMDR_SPEC> {
        FEF_W::new(self, 7)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    #[must_use]
    pub fn stt(&mut self) -> STT_W<DMAMDR_SPEC> {
        STT_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<DMAMDR_SPEC> {
        TTC_W::new(self, 14)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<DMAMDR_SPEC> {
        FTF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<DMAMDR_SPEC> {
        TSF_W::new(self, 21)
    }
    #[doc = "Bit 24 - When this bit is set to 1, RxDMA will not refresh any frame because the receive descriptor/buffer is unavailable, please refer to the section \"Receiving Process Suspended\""]
    #[inline(always)]
    #[must_use]
    pub fn dfrf(&mut self) -> DFRF_W<DMAMDR_SPEC> {
        DFRF_W::new(self, 24)
    }
    #[doc = "Bit 25 - After the RxFIFO writes a complete frame, it executes the read action while ignoring the RTC bit"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<DMAMDR_SPEC> {
        RSF_W::new(self, 25)
    }
    #[doc = "Bit 26 - If there are only errors detected by the receive checksum offload engine in the received frame, the kernel will not discard the frame"]
    #[inline(always)]
    #[must_use]
    pub fn dtcoe(&mut self) -> DTCOE_W<DMAMDR_SPEC> {
        DTCOE_W::new(self, 26)
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
#[doc = "Ethernet DMA operation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMDR_SPEC;
impl crate::RegisterSpec for DMAMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamdr::R`](R) reader structure"]
impl crate::Readable for DMAMDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmamdr::W`](W) writer structure"]
impl crate::Writable for DMAMDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMDR to value 0"]
impl crate::Resettable for DMAMDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
