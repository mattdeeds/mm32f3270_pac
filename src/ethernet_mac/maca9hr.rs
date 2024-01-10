#[doc = "Register `MACA9HR` reader"]
pub type R = crate::R<MACA9HR_SPEC>;
#[doc = "Register `MACA9HR` writer"]
pub type W = crate::W<MACA9HR_SPEC>;
#[doc = "Field `ADDH` reader - MAC address9 high \\[47:32\\]"]
pub type ADDH_R = crate::FieldReader<u16>;
#[doc = "Field `ADDH` writer - MAC address9 high \\[47:32\\]"]
pub type ADDH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBYTEC` reader - Mask Byte Control"]
pub type MBYTEC_R = crate::FieldReader;
#[doc = "Field `MBYTEC` writer - Mask Byte Control"]
pub type MBYTEC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SELE` reader - Source address"]
pub type SELE_R = crate::BitReader;
#[doc = "Field `SELE` writer - Source address"]
pub type SELE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDE` reader - Address enable"]
pub type ADDE_R = crate::BitReader;
#[doc = "Field `ADDE` writer - Address enable"]
pub type ADDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC address9 high \\[47:32\\]"]
    #[inline(always)]
    pub fn addh(&self) -> ADDH_R {
        ADDH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn mbytec(&self) -> MBYTEC_R {
        MBYTEC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sele(&self) -> SELE_R {
        SELE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn adde(&self) -> ADDE_R {
        ADDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address9 high \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addh(&mut self) -> ADDH_W<MACA9HR_SPEC> {
        ADDH_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    #[must_use]
    pub fn mbytec(&mut self) -> MBYTEC_W<MACA9HR_SPEC> {
        MBYTEC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sele(&mut self) -> SELE_W<MACA9HR_SPEC> {
        SELE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn adde(&mut self) -> ADDE_W<MACA9HR_SPEC> {
        ADDE_W::new(self, 31)
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
#[doc = "Ethernet MAC address 9 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca9hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca9hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA9HR_SPEC;
impl crate::RegisterSpec for MACA9HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca9hr::R`](R) reader structure"]
impl crate::Readable for MACA9HR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca9hr::W`](W) writer structure"]
impl crate::Writable for MACA9HR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA9HR to value 0"]
impl crate::Resettable for MACA9HR_SPEC {
    const RESET_VALUE: u32 = 0;
}
