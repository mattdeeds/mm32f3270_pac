#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `OPMSEL` reader - SD/MMC/SDIO port operation mode selection"]
pub type OPMSEL_R = crate::BitReader;
#[doc = "Field `OPMSEL` writer - SD/MMC/SDIO port operation mode selection"]
pub type OPMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSEL` reader - Signal mode selection"]
pub type SMSEL_R = crate::BitReader;
#[doc = "Field `SMSEL` writer - Signal mode selection"]
pub type SMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTM` reader - SD/MMC/SDIO port CMD line output drive mode selection"]
pub type OUTM_R = crate::BitReader;
#[doc = "Field `OUTM` writer - SD/MMC/SDIO port CMD line output drive mode selection"]
pub type OUTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` reader - SD/MMC/SDIO port CLK line speed selection"]
pub type CLKSEL_R = crate::FieldReader;
#[doc = "Field `CLKSEL` writer - SD/MMC/SDIO port CLK line speed selection"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PTSMSEL` reader - SD/MMC/SDIO port transfer speed mode selection"]
pub type PTSMSEL_R = crate::BitReader;
#[doc = "Field `PTSMSEL` writer - SD/MMC/SDIO port transfer speed mode selection"]
pub type PTSMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATWT` reader - Define the bus width of SD/MMC/SDIO port DAT line"]
pub type DATWT_R = crate::BitReader;
#[doc = "Field `DATWT` writer - Define the bus width of SD/MMC/SDIO port DAT line"]
pub type DATWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - SDIO mode enable"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - SDIO mode enable"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - SDIO interrupt enable signal"]
pub type INTEN_R = crate::BitReader;
#[doc = "Field `INTEN` writer - SDIO interrupt enable signal"]
pub type INTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWTEN` reader - SDIO read wait enable signal"]
pub type RDWTEN_R = crate::BitReader;
#[doc = "Field `RDWTEN` writer - SDIO read wait enable signal"]
pub type RDWTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SD/MMC/SDIO port operation mode selection"]
    #[inline(always)]
    pub fn opmsel(&self) -> OPMSEL_R {
        OPMSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Signal mode selection"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD/MMC/SDIO port CMD line output drive mode selection"]
    #[inline(always)]
    pub fn outm(&self) -> OUTM_R {
        OUTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - SD/MMC/SDIO port CLK line speed selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port transfer speed mode selection"]
    #[inline(always)]
    pub fn ptsmsel(&self) -> PTSMSEL_R {
        PTSMSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Define the bus width of SD/MMC/SDIO port DAT line"]
    #[inline(always)]
    pub fn datwt(&self) -> DATWT_R {
        DATWT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO mode enable"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO interrupt enable signal"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO read wait enable signal"]
    #[inline(always)]
    pub fn rdwten(&self) -> RDWTEN_R {
        RDWTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SD/MMC/SDIO port operation mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn opmsel(&mut self) -> OPMSEL_W<CR1_SPEC> {
        OPMSEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Signal mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smsel(&mut self) -> SMSEL_W<CR1_SPEC> {
        SMSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - SD/MMC/SDIO port CMD line output drive mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn outm(&mut self) -> OUTM_W<CR1_SPEC> {
        OUTM_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - SD/MMC/SDIO port CLK line speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CR1_SPEC> {
        CLKSEL_W::new(self, 3)
    }
    #[doc = "Bit 6 - SD/MMC/SDIO port transfer speed mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptsmsel(&mut self) -> PTSMSEL_W<CR1_SPEC> {
        PTSMSEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Define the bus width of SD/MMC/SDIO port DAT line"]
    #[inline(always)]
    #[must_use]
    pub fn datwt(&mut self) -> DATWT_W<CR1_SPEC> {
        DATWT_W::new(self, 7)
    }
    #[doc = "Bit 8 - SDIO mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CR1_SPEC> {
        MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - SDIO interrupt enable signal"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<CR1_SPEC> {
        INTEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - SDIO read wait enable signal"]
    #[inline(always)]
    #[must_use]
    pub fn rdwten(&mut self) -> RDWTEN_W<CR1_SPEC> {
        RDWTEN_W::new(self, 10)
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
#[doc = "CR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x45"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: u32 = 0x45;
}
