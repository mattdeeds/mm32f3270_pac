#[doc = "Register `TAR` reader"]
pub type R = crate::R<TAR_SPEC>;
#[doc = "Register `TAR` writer"]
pub type W = crate::W<TAR_SPEC>;
#[doc = "Field `ADDR` reader - This is the target address for any master transaction"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - This is the target address for any master transaction"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `GC` reader - If bit 11(SPECIAL)is set to 1"]
pub type GC_R = crate::BitReader;
#[doc = "Field `GC` writer - If bit 11(SPECIAL)is set to 1"]
pub type GC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPECIAL` reader - This bit indicates whether software performs a General Call or START BYTE conmmend"]
pub type SPECIAL_R = crate::BitReader;
#[doc = "Field `SPECIAL` writer - This bit indicates whether software performs a General Call or START BYTE conmmend"]
pub type SPECIAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - This is the target address for any master transaction"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - If bit 11(SPECIAL)is set to 1"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE conmmend"]
    #[inline(always)]
    pub fn special(&self) -> SPECIAL_R {
        SPECIAL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This is the target address for any master transaction"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<TAR_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 10 - If bit 11(SPECIAL)is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GC_W<TAR_SPEC> {
        GC_W::new(self, 10)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE conmmend"]
    #[inline(always)]
    #[must_use]
    pub fn special(&mut self) -> SPECIAL_W<TAR_SPEC> {
        SPECIAL_W::new(self, 11)
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
#[doc = "Target Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAR to value 0x55"]
impl crate::Resettable for TAR_SPEC {
    const RESET_VALUE: u32 = 0x55;
}
