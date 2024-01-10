#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLYC` reader - Deferral check"]
pub type DLYC_R = crate::BitReader;
#[doc = "Field `DLYC` writer - Deferral check"]
pub type DLYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APCS` reader - Automatic pad/CRC stripping"]
pub type APCS_R = crate::BitReader;
#[doc = "Field `APCS` writer - Automatic pad/CRC stripping"]
pub type APCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETY` reader - Retry disable"]
pub type RETY_R = crate::BitReader;
#[doc = "Field `RETY` writer - Retry disable"]
pub type RETY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCO` reader - IPv4 Checksum Offload"]
pub type IPCO_R = crate::BitReader;
#[doc = "Field `IPCO` writer - IPv4 Checksum Offload"]
pub type IPCO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback Mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loopback Mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROD` reader - Receive Own Disable"]
pub type ROD_R = crate::BitReader;
#[doc = "Field `ROD` writer - Receive Own Disable"]
pub type ROD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Fast Ethernet Speed"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - Fast Ethernet Speed"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCRS` reader - Carrier sense disable"]
pub type MCRS_R = crate::BitReader;
#[doc = "Field `MCRS` writer - Carrier sense disable"]
pub type MCRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Inter Frame Gap"]
pub type IFG_R = crate::FieldReader;
#[doc = "Field `IFG` writer - Inter Frame Gap"]
pub type IFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JD` reader - Jabber Disable"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - Jabber Disable"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTD` reader - Watchdog Disable"]
pub type WTD_R = crate::BitReader;
#[doc = "Field `WTD` writer - Watchdog Disable"]
pub type WTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CST` reader - CRC stripping for Type frames"]
pub type CST_R = crate::BitReader;
#[doc = "Field `CST` writer - CRC stripping for Type frames"]
pub type CST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dlyc(&self) -> DLYC_R {
        DLYC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rety(&self) -> RETY_R {
        RETY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 Checksum Offload"]
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Own Disable"]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Ethernet Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn mcrs(&self) -> MCRS_R {
        MCRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter Frame Gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn wtd(&self) -> WTD_R {
        WTD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CRC stripping for Type frames"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<CR_SPEC> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<CR_SPEC> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    #[must_use]
    pub fn dlyc(&mut self) -> DLYC_W<CR_SPEC> {
        DLYC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<CR_SPEC> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<CR_SPEC> {
        APCS_W::new(self, 7)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    #[must_use]
    pub fn rety(&mut self) -> RETY_W<CR_SPEC> {
        RETY_W::new(self, 9)
    }
    #[doc = "Bit 10 - IPv4 Checksum Offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<CR_SPEC> {
        IPCO_W::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<CR_SPEC> {
        DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<CR_SPEC> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Receive Own Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<CR_SPEC> {
        ROD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Ethernet Speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<CR_SPEC> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    #[must_use]
    pub fn mcrs(&mut self) -> MCRS_W<CR_SPEC> {
        MCRS_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Inter Frame Gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<CR_SPEC> {
        IFG_W::new(self, 17)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<CR_SPEC> {
        JD_W::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wtd(&mut self) -> WTD_W<CR_SPEC> {
        WTD_W::new(self, 23)
    }
    #[doc = "Bit 25 - CRC stripping for Type frames"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<CR_SPEC> {
        CST_W::new(self, 25)
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
#[doc = "Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x8000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
