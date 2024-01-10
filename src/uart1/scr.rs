#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCR_SPEC>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `SCEN` reader - ISO7816 enable control bit"]
pub type SCEN_R = crate::BitReader;
#[doc = "Field `SCEN` writer - ISO7816 enable control bit"]
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAEN` reader - ISO7816 check auto-response bit"]
pub type SCAEN_R = crate::BitReader;
#[doc = "Field `SCAEN` writer - ISO7816 check auto-response bit"]
pub type SCAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Master receive frame answer bit"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `SCFCNT` reader - ISO7816 protection counter bit"]
pub type SCFCNT_R = crate::FieldReader;
#[doc = "Field `SCFCNT` writer - ISO7816 protection counter bit"]
pub type SCFCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HDSEL` reader - Single-wire half-duplex mode selection bit"]
pub type HDSEL_R = crate::BitReader;
#[doc = "Field `HDSEL` writer - Single-wire half-duplex mode selection bit"]
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ISO7816 enable control bit"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO7816 check auto-response bit"]
    #[inline(always)]
    pub fn scaen(&self) -> SCAEN_R {
        SCAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master receive frame answer bit"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:11 - ISO7816 protection counter bit"]
    #[inline(always)]
    pub fn scfcnt(&self) -> SCFCNT_R {
        SCFCNT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Single-wire half-duplex mode selection bit"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISO7816 enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<SCR_SPEC> {
        SCEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ISO7816 check auto-response bit"]
    #[inline(always)]
    #[must_use]
    pub fn scaen(&mut self) -> SCAEN_W<SCR_SPEC> {
        SCAEN_W::new(self, 1)
    }
    #[doc = "Bits 4:11 - ISO7816 protection counter bit"]
    #[inline(always)]
    #[must_use]
    pub fn scfcnt(&mut self) -> SCFCNT_W<SCR_SPEC> {
        SCFCNT_W::new(self, 4)
    }
    #[doc = "Bit 12 - Single-wire half-duplex mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<SCR_SPEC> {
        HDSEL_W::new(self, 12)
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
#[doc = "ISO7816 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
