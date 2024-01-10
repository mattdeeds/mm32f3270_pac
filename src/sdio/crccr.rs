#[doc = "Register `CRCCR` reader"]
pub type R = crate::R<CRCCR_SPEC>;
#[doc = "Register `CRCCR` writer"]
pub type W = crate::W<CRCCR_SPEC>;
#[doc = "Field `DAT_CRCE` reader - DAT CRC Error"]
pub type DAT_CRCE_R = crate::BitReader;
#[doc = "Field `DAT_CRCE` writer - DAT CRC Error"]
pub type DAT_CRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRCE` reader - CMD CRC Error"]
pub type CMD_CRCE_R = crate::BitReader;
#[doc = "Field `CMD_CRCE` writer - CMD CRC Error"]
pub type CMD_CRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_CRCS` reader - DAT CRC selection"]
pub type DAT_CRCS_R = crate::FieldReader;
#[doc = "Field `DAT_CRCS` writer - DAT CRC selection"]
pub type DAT_CRCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RDMBEN` reader - Enable read multiple block data before response"]
pub type RDMBEN_R = crate::BitReader;
#[doc = "Field `RDMBEN` writer - Enable read multiple block data before response"]
pub type RDMBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHKEN` reader - Enable auto check crc_status\\[2:0\\]"]
pub type CHKEN_R = crate::BitReader;
#[doc = "Field `CHKEN` writer - Enable auto check crc_status\\[2:0\\]"]
pub type CHKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_CRCEN` reader - SD/MMC/SDIO port DAT Line CRC circuit enable"]
pub type DAT_CRCEN_R = crate::BitReader;
#[doc = "Field `DAT_CRCEN` writer - SD/MMC/SDIO port DAT Line CRC circuit enable"]
pub type DAT_CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRCEN` reader - SD/MMC/SDIO port CMD Line CRC circuit enable."]
pub type CMD_CRCEN_R = crate::BitReader;
#[doc = "Field `CMD_CRCEN` writer - SD/MMC/SDIO port CMD Line CRC circuit enable."]
pub type CMD_CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAT CRC Error"]
    #[inline(always)]
    pub fn dat_crce(&self) -> DAT_CRCE_R {
        DAT_CRCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD CRC Error"]
    #[inline(always)]
    pub fn cmd_crce(&self) -> CMD_CRCE_R {
        CMD_CRCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DAT CRC selection"]
    #[inline(always)]
    pub fn dat_crcs(&self) -> DAT_CRCS_R {
        DAT_CRCS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable read multiple block data before response"]
    #[inline(always)]
    pub fn rdmben(&self) -> RDMBEN_R {
        RDMBEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable auto check crc_status\\[2:0\\]"]
    #[inline(always)]
    pub fn chken(&self) -> CHKEN_R {
        CHKEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port DAT Line CRC circuit enable"]
    #[inline(always)]
    pub fn dat_crcen(&self) -> DAT_CRCEN_R {
        DAT_CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO port CMD Line CRC circuit enable."]
    #[inline(always)]
    pub fn cmd_crcen(&self) -> CMD_CRCEN_R {
        CMD_CRCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAT CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn dat_crce(&mut self) -> DAT_CRCE_W<CRCCR_SPEC> {
        DAT_CRCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CMD CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crce(&mut self) -> CMD_CRCE_W<CRCCR_SPEC> {
        CMD_CRCE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - DAT CRC selection"]
    #[inline(always)]
    #[must_use]
    pub fn dat_crcs(&mut self) -> DAT_CRCS_W<CRCCR_SPEC> {
        DAT_CRCS_W::new(self, 2)
    }
    #[doc = "Bit 4 - Enable read multiple block data before response"]
    #[inline(always)]
    #[must_use]
    pub fn rdmben(&mut self) -> RDMBEN_W<CRCCR_SPEC> {
        RDMBEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable auto check crc_status\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn chken(&mut self) -> CHKEN_W<CRCCR_SPEC> {
        CHKEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port DAT Line CRC circuit enable"]
    #[inline(always)]
    #[must_use]
    pub fn dat_crcen(&mut self) -> DAT_CRCEN_W<CRCCR_SPEC> {
        DAT_CRCEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - SD/MMC/SDIO port CMD Line CRC circuit enable."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crcen(&mut self) -> CMD_CRCEN_W<CRCCR_SPEC> {
        CMD_CRCEN_W::new(self, 7)
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
#[doc = "CRC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCCR_SPEC;
impl crate::RegisterSpec for CRCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crccr::R`](R) reader structure"]
impl crate::Readable for CRCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crccr::W`](W) writer structure"]
impl crate::Writable for CRCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCCR to value 0"]
impl crate::Resettable for CRCCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
