#[doc = "Register `SFF_P` reader"]
pub type R = crate::R<SFF_P_SPEC>;
#[doc = "Register `SFF_P` writer"]
pub type W = crate::W<SFF_P_SPEC>;
#[doc = "Field `DLC` reader - Data length code bit"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - Data length code bit"]
pub type DLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTR` reader - Remote transmission request"]
pub type RTR_R = crate::BitReader;
#[doc = "Field `RTR` writer - Remote transmission request"]
pub type RTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader;
#[doc = "Field `FF` writer - Frame format"]
pub type FF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data length code bit"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code bit"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<SFF_P_SPEC> {
        DLC_W::new(self, 0)
    }
    #[doc = "Bit 6 - Remote transmission request"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<SFF_P_SPEC> {
        RTR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FF_W<SFF_P_SPEC> {
        FF_W::new(self, 7)
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
#[doc = "Peli Send Frame Format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sff_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sff_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFF_P_SPEC;
impl crate::RegisterSpec for SFF_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sff_p::R`](R) reader structure"]
impl crate::Readable for SFF_P_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sff_p::W`](W) writer structure"]
impl crate::Writable for SFF_P_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SFF_P to value 0"]
impl crate::Resettable for SFF_P_SPEC {
    const RESET_VALUE: u32 = 0;
}
