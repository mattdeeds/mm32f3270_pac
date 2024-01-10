#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<ADDR_SPEC>;
#[doc = "Field `ADDR` reader - The USB address of the host is defined in the usb_7 mode"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - The USB address of the host is defined in the usb_7 mode"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LS_EN` reader - Enabling this bit tells otg_fs that the next token command to write to the token register must be executed at a low speed."]
pub type LS_EN_R = crate::BitReader;
#[doc = "Field `LS_EN` writer - Enabling this bit tells otg_fs that the next token command to write to the token register must be executed at a low speed."]
pub type LS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - The USB address of the host is defined in the usb_7 mode"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Enabling this bit tells otg_fs that the next token command to write to the token register must be executed at a low speed."]
    #[inline(always)]
    pub fn ls_en(&self) -> LS_EN_R {
        LS_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The USB address of the host is defined in the usb_7 mode"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<ADDR_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 7 - Enabling this bit tells otg_fs that the next token command to write to the token register must be executed at a low speed."]
    #[inline(always)]
    #[must_use]
    pub fn ls_en(&mut self) -> LS_EN_W<ADDR_SPEC> {
        LS_EN_W::new(self, 7)
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
#[doc = "Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
