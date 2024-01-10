#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `T` reader - 7-bit counter"]
pub type T_R = crate::FieldReader;
#[doc = "Field `T` writer - 7-bit counter"]
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGA` reader - Activation bit"]
pub type WDGA_R = crate::BitReader;
#[doc = "Field `WDGA` writer - Activation bit"]
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> T_W<CR_SPEC> {
        T_W::new(self, 0)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WDGA_W<CR_SPEC> {
        WDGA_W::new(self, 7)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x7f"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
