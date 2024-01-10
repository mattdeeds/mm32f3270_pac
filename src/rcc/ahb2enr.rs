#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENR_SPEC>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENR_SPEC>;
#[doc = "Field `USBOTGFS` reader - *D7"]
pub type USBOTGFS_R = crate::BitReader;
#[doc = "Field `USBOTGFS` writer - *D7"]
pub type USBOTGFS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - *D7"]
    #[inline(always)]
    pub fn usbotgfs(&self) -> USBOTGFS_R {
        USBOTGFS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - *D7"]
    #[inline(always)]
    #[must_use]
    pub fn usbotgfs(&mut self) -> USBOTGFS_W<AHB2ENR_SPEC> {
        USBOTGFS_W::new(self, 7)
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
#[doc = "Advanced High Performance Bus 2 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: u32 = 0;
}
