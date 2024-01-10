#[doc = "Register `COMP_CRV` reader"]
pub type R = crate::R<COMP_CRV_SPEC>;
#[doc = "Register `COMP_CRV` writer"]
pub type W = crate::W<COMP_CRV_SPEC>;
#[doc = "Field `CRV_SEL` reader - Comparator external reference voltage select"]
pub type CRV_SEL_R = crate::FieldReader;
#[doc = "Field `CRV_SEL` writer - Comparator external reference voltage select"]
pub type CRV_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CRV_EN` reader - Comparator external reference voltage enable"]
pub type CRV_EN_R = crate::BitReader;
#[doc = "Field `CRV_EN` writer - Comparator external reference voltage enable"]
pub type CRV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRV_SRC` reader - Comparator external reference voltage source select"]
pub type CRV_SRC_R = crate::BitReader;
#[doc = "Field `CRV_SRC` writer - Comparator external reference voltage source select"]
pub type CRV_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Comparator external reference voltage select"]
    #[inline(always)]
    pub fn crv_sel(&self) -> CRV_SEL_R {
        CRV_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Comparator external reference voltage enable"]
    #[inline(always)]
    pub fn crv_en(&self) -> CRV_EN_R {
        CRV_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator external reference voltage source select"]
    #[inline(always)]
    pub fn crv_src(&self) -> CRV_SRC_R {
        CRV_SRC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comparator external reference voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn crv_sel(&mut self) -> CRV_SEL_W<COMP_CRV_SPEC> {
        CRV_SEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Comparator external reference voltage enable"]
    #[inline(always)]
    #[must_use]
    pub fn crv_en(&mut self) -> CRV_EN_W<COMP_CRV_SPEC> {
        CRV_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Comparator external reference voltage source select"]
    #[inline(always)]
    #[must_use]
    pub fn crv_src(&mut self) -> CRV_SRC_W<COMP_CRV_SPEC> {
        CRV_SRC_W::new(self, 5)
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
#[doc = "COMP Extern Reference Voltage\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_crv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_crv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_CRV_SPEC;
impl crate::RegisterSpec for COMP_CRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_crv::R`](R) reader structure"]
impl crate::Readable for COMP_CRV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comp_crv::W`](W) writer structure"]
impl crate::Writable for COMP_CRV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP_CRV to value 0"]
impl crate::Resettable for COMP_CRV_SPEC {
    const RESET_VALUE: u32 = 0;
}
