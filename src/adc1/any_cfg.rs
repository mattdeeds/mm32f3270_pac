#[doc = "Register `ANY_CFG` reader"]
pub type R = crate::R<ANY_CFG_SPEC>;
#[doc = "Register `ANY_CFG` writer"]
pub type W = crate::W<ANY_CFG_SPEC>;
#[doc = "Field `CHANY_NUM` reader - channel number configuration"]
pub type CHANY_NUM_R = crate::FieldReader;
#[doc = "Field `CHANY_NUM` writer - channel number configuration"]
pub type CHANY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - channel number configuration"]
    #[inline(always)]
    pub fn chany_num(&self) -> CHANY_NUM_R {
        CHANY_NUM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - channel number configuration"]
    #[inline(always)]
    #[must_use]
    pub fn chany_num(&mut self) -> CHANY_NUM_W<ANY_CFG_SPEC> {
        CHANY_NUM_W::new(self, 0)
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
#[doc = "Arbitrary channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`any_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`any_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANY_CFG_SPEC;
impl crate::RegisterSpec for ANY_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`any_cfg::R`](R) reader structure"]
impl crate::Readable for ANY_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`any_cfg::W`](W) writer structure"]
impl crate::Writable for ANY_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANY_CFG to value 0"]
impl crate::Resettable for ANY_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
