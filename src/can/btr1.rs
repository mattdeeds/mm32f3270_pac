#[doc = "Register `BTR1` reader"]
pub type R = crate::R<BTR1_SPEC>;
#[doc = "Register `BTR1` writer"]
pub type W = crate::W<BTR1_SPEC>;
#[doc = "Field `TSEG1` reader - Time segment 1"]
pub type TSEG1_R = crate::FieldReader;
#[doc = "Field `TSEG1` writer - Time segment 1"]
pub type TSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSEG2` reader - Time segment 2"]
pub type TSEG2_R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Time segment 2"]
pub type TSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAM` reader - Sampling"]
pub type SAM_R = crate::BitReader;
#[doc = "Field `SAM` writer - Sampling"]
pub type SAM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Time segment 1"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Time segment 2"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Sampling"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn tseg1(&mut self) -> TSEG1_W<BTR1_SPEC> {
        TSEG1_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Time segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<BTR1_SPEC> {
        TSEG2_W::new(self, 4)
    }
    #[doc = "Bit 7 - Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn sam(&mut self) -> SAM_W<BTR1_SPEC> {
        SAM_W::new(self, 7)
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
#[doc = "Bus Timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTR1_SPEC;
impl crate::RegisterSpec for BTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr1::R`](R) reader structure"]
impl crate::Readable for BTR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btr1::W`](W) writer structure"]
impl crate::Writable for BTR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTR1 to value 0"]
impl crate::Resettable for BTR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
