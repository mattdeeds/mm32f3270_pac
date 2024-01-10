#[doc = "Register `SIGR` reader"]
pub type R = crate::R<SIGR_SPEC>;
#[doc = "Field `PDAT0S` reader - SD/MMC/SDIO port DAT0 Line signal"]
pub type PDAT0S_R = crate::BitReader;
#[doc = "Field `PDAT1S` reader - SD/MMC/SDIO port DAT1 Line signal"]
pub type PDAT1S_R = crate::BitReader;
#[doc = "Field `PDAT2S` reader - SD/MMC/SDIO port DAT2 Line signal"]
pub type PDAT2S_R = crate::BitReader;
#[doc = "Field `PDAT3S` reader - SD/MMC/SDIO port DAT3 Line signal"]
pub type PDAT3S_R = crate::BitReader;
#[doc = "Field `CRCST` reader - CRC status\\[2:0\\]
when write data CRC status token"]
pub type CRCST_R = crate::FieldReader;
#[doc = "Field `PCMDS` reader - SD/MMC/SDIO port CMD Line signal"]
pub type PCMDS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SD/MMC/SDIO port DAT0 Line signal"]
    #[inline(always)]
    pub fn pdat0s(&self) -> PDAT0S_R {
        PDAT0S_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SD/MMC/SDIO port DAT1 Line signal"]
    #[inline(always)]
    pub fn pdat1s(&self) -> PDAT1S_R {
        PDAT1S_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD/MMC/SDIO port DAT2 Line signal"]
    #[inline(always)]
    pub fn pdat2s(&self) -> PDAT2S_R {
        PDAT2S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD/MMC/SDIO port DAT3 Line signal"]
    #[inline(always)]
    pub fn pdat3s(&self) -> PDAT3S_R {
        PDAT3S_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - CRC status\\[2:0\\]
when write data CRC status token"]
    #[inline(always)]
    pub fn crcst(&self) -> CRCST_R {
        CRCST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO port CMD Line signal"]
    #[inline(always)]
    pub fn pcmds(&self) -> PCMDS_R {
        PCMDS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Signal register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGR_SPEC;
impl crate::RegisterSpec for SIGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigr::R`](R) reader structure"]
impl crate::Readable for SIGR_SPEC {}
#[doc = "`reset()` method sets SIGR to value 0xff"]
impl crate::Resettable for SIGR_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
