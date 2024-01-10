#[doc = "Register `PORT` reader"]
pub type R = crate::R<PORT_SPEC>;
#[doc = "Field `NTCR` reader - Ncr Timeout count register(SD/MMC/SDIO clock number)"]
pub type NTCR_R = crate::FieldReader;
#[doc = "Field `AUTONTEN` reader - Auto Ncr Timer out enable"]
pub type AUTONTEN_R = crate::BitReader;
#[doc = "Field `PDATS` reader - SD/MMC/SDIO port DAT line signal"]
pub type PDATS_R = crate::BitReader;
#[doc = "Field `PCMDS` reader - SD/MMC/SDIO port CMD line signal"]
pub type PCMDS_R = crate::BitReader;
#[doc = "Field `PCLKS` reader - SD/MMC/SDIO port CLK line signal"]
pub type PCLKS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Ncr Timeout count register(SD/MMC/SDIO clock number)"]
    #[inline(always)]
    pub fn ntcr(&self) -> NTCR_R {
        NTCR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Auto Ncr Timer out enable"]
    #[inline(always)]
    pub fn autonten(&self) -> AUTONTEN_R {
        AUTONTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SD/MMC/SDIO port DAT line signal"]
    #[inline(always)]
    pub fn pdats(&self) -> PDATS_R {
        PDATS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port CMD line signal"]
    #[inline(always)]
    pub fn pcmds(&self) -> PCMDS_R {
        PCMDS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO port CLK line signal"]
    #[inline(always)]
    pub fn pclks(&self) -> PCLKS_R {
        PCLKS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "MMC port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`port::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORT_SPEC;
impl crate::RegisterSpec for PORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port::R`](R) reader structure"]
impl crate::Readable for PORT_SPEC {}
#[doc = "`reset()` method sets PORT to value 0x7f"]
impl crate::Resettable for PORT_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
