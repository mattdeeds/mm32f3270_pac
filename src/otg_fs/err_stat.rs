#[doc = "Register `ERR_STAT` reader"]
pub type R = crate::R<ERR_STAT_SPEC>;
#[doc = "Register `ERR_STAT` writer"]
pub type W = crate::W<ERR_STAT_SPEC>;
#[doc = "Field `PID_ERR` reader - PID check field failed"]
pub type PID_ERR_R = crate::BitReader;
#[doc = "Field `PID_ERR` writer - PID check field failed"]
pub type PID_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC5_EOF` reader - This error interrupt has two functions"]
pub type CRC5_EOF_R = crate::BitReader;
#[doc = "Field `CRC5_EOF` writer - This error interrupt has two functions"]
pub type CRC5_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16` reader - CRC16 failed"]
pub type CRC16_R = crate::BitReader;
#[doc = "Field `CRC16` writer - CRC16 failed"]
pub type CRC16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFN8` reader - The received data field is not 8 bits"]
pub type DFN8_R = crate::BitReader;
#[doc = "Field `DFN8` writer - The received data field is not 8 bits"]
pub type DFN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTO_ERR` reader - If a bus turnaround timeout error occurs, the location bit"]
pub type BTO_ERR_R = crate::BitReader;
#[doc = "Field `BTO_ERR` writer - If a bus turnaround timeout error occurs, the location bit"]
pub type BTO_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_ERR` reader - If DMA error,this position 1"]
pub type DMA_ERR_R = crate::BitReader;
#[doc = "Field `DMA_ERR` writer - If DMA error,this position 1"]
pub type DMA_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTS_ERR` reader - Bit stuff is set when the error is detected. The corresponding packet will be rejected"]
pub type BTS_ERR_R = crate::BitReader;
#[doc = "Field `BTS_ERR` writer - Bit stuff is set when the error is detected. The corresponding packet will be rejected"]
pub type BTS_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PID check field failed"]
    #[inline(always)]
    pub fn pid_err(&self) -> PID_ERR_R {
        PID_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This error interrupt has two functions"]
    #[inline(always)]
    pub fn crc5_eof(&self) -> CRC5_EOF_R {
        CRC5_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC16 failed"]
    #[inline(always)]
    pub fn crc16(&self) -> CRC16_R {
        CRC16_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The received data field is not 8 bits"]
    #[inline(always)]
    pub fn dfn8(&self) -> DFN8_R {
        DFN8_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If a bus turnaround timeout error occurs, the location bit"]
    #[inline(always)]
    pub fn bto_err(&self) -> BTO_ERR_R {
        BTO_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If DMA error,this position 1"]
    #[inline(always)]
    pub fn dma_err(&self) -> DMA_ERR_R {
        DMA_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit stuff is set when the error is detected. The corresponding packet will be rejected"]
    #[inline(always)]
    pub fn bts_err(&self) -> BTS_ERR_R {
        BTS_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PID check field failed"]
    #[inline(always)]
    #[must_use]
    pub fn pid_err(&mut self) -> PID_ERR_W<ERR_STAT_SPEC> {
        PID_ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - This error interrupt has two functions"]
    #[inline(always)]
    #[must_use]
    pub fn crc5_eof(&mut self) -> CRC5_EOF_W<ERR_STAT_SPEC> {
        CRC5_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - CRC16 failed"]
    #[inline(always)]
    #[must_use]
    pub fn crc16(&mut self) -> CRC16_W<ERR_STAT_SPEC> {
        CRC16_W::new(self, 2)
    }
    #[doc = "Bit 3 - The received data field is not 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn dfn8(&mut self) -> DFN8_W<ERR_STAT_SPEC> {
        DFN8_W::new(self, 3)
    }
    #[doc = "Bit 4 - If a bus turnaround timeout error occurs, the location bit"]
    #[inline(always)]
    #[must_use]
    pub fn bto_err(&mut self) -> BTO_ERR_W<ERR_STAT_SPEC> {
        BTO_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - If DMA error,this position 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_err(&mut self) -> DMA_ERR_W<ERR_STAT_SPEC> {
        DMA_ERR_W::new(self, 5)
    }
    #[doc = "Bit 7 - Bit stuff is set when the error is detected. The corresponding packet will be rejected"]
    #[inline(always)]
    #[must_use]
    pub fn bts_err(&mut self) -> BTS_ERR_W<ERR_STAT_SPEC> {
        BTS_ERR_W::new(self, 7)
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
#[doc = "Error interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_STAT_SPEC;
impl crate::RegisterSpec for ERR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat::R`](R) reader structure"]
impl crate::Readable for ERR_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`err_stat::W`](W) writer structure"]
impl crate::Writable for ERR_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_STAT to value 0"]
impl crate::Resettable for ERR_STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
