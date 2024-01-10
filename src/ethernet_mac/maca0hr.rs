#[doc = "Register `MACA0HR` reader"]
pub type R = crate::R<MACA0HR_SPEC>;
#[doc = "Register `MACA0HR` writer"]
pub type W = crate::W<MACA0HR_SPEC>;
#[doc = "Field `ADDH` reader - MAC address0 high \\[47:32\\]"]
pub type ADDH_R = crate::FieldReader<u16>;
#[doc = "Field `ADDH` writer - MAC address0 high \\[47:32\\]"]
pub type ADDH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDE` reader - Always 1"]
pub type ADDE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high \\[47:32\\]"]
    #[inline(always)]
    pub fn addh(&self) -> ADDH_R {
        ADDH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Always 1"]
    #[inline(always)]
    pub fn adde(&self) -> ADDE_R {
        ADDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addh(&mut self) -> ADDH_W<MACA0HR_SPEC> {
        ADDH_W::new(self, 0)
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
#[doc = "Ethernet MAC address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA0HR_SPEC;
impl crate::RegisterSpec for MACA0HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0hr::R`](R) reader structure"]
impl crate::Readable for MACA0HR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca0hr::W`](W) writer structure"]
impl crate::Writable for MACA0HR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA0HR to value 0"]
impl crate::Resettable for MACA0HR_SPEC {
    const RESET_VALUE: u32 = 0;
}
