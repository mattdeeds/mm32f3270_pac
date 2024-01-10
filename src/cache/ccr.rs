#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `EN` reader - Enable cache"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable cache"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV_REQ` reader - Invalid manual set request"]
pub type INV_REQ_R = crate::BitReader;
#[doc = "Field `INV_REQ` writer - Invalid manual set request"]
pub type INV_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POW_REQ` reader - Manual SRAM power request"]
pub type POW_REQ_R = crate::BitReader;
#[doc = "Field `POW_REQ` writer - Manual SRAM power request"]
pub type POW_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_MAN_POW` reader - Set manual or automatic SRAM power request"]
pub type SET_MAN_POW_R = crate::BitReader;
#[doc = "Field `SET_MAN_POW` writer - Set manual or automatic SRAM power request"]
pub type SET_MAN_POW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_MAN_INV` reader - Set manual or automatic invalid"]
pub type SET_MAN_INV_R = crate::BitReader;
#[doc = "Field `SET_MAN_INV` writer - Set manual or automatic invalid"]
pub type SET_MAN_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_PREFETCH` reader - Prefetch value"]
pub type SET_PREFETCH_R = crate::BitReader;
#[doc = "Field `SET_PREFETCH` writer - Prefetch value"]
pub type SET_PREFETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATISTIC_EN` reader - Enable statistics"]
pub type STATISTIC_EN_R = crate::BitReader;
#[doc = "Field `STATISTIC_EN` writer - Enable statistics"]
pub type STATISTIC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable cache"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid manual set request"]
    #[inline(always)]
    pub fn inv_req(&self) -> INV_REQ_R {
        INV_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Manual SRAM power request"]
    #[inline(always)]
    pub fn pow_req(&self) -> POW_REQ_R {
        POW_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set manual or automatic SRAM power request"]
    #[inline(always)]
    pub fn set_man_pow(&self) -> SET_MAN_POW_R {
        SET_MAN_POW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set manual or automatic invalid"]
    #[inline(always)]
    pub fn set_man_inv(&self) -> SET_MAN_INV_R {
        SET_MAN_INV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch value"]
    #[inline(always)]
    pub fn set_prefetch(&self) -> SET_PREFETCH_R {
        SET_PREFETCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable statistics"]
    #[inline(always)]
    pub fn statistic_en(&self) -> STATISTIC_EN_R {
        STATISTIC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable cache"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CCR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Invalid manual set request"]
    #[inline(always)]
    #[must_use]
    pub fn inv_req(&mut self) -> INV_REQ_W<CCR_SPEC> {
        INV_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Manual SRAM power request"]
    #[inline(always)]
    #[must_use]
    pub fn pow_req(&mut self) -> POW_REQ_W<CCR_SPEC> {
        POW_REQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set manual or automatic SRAM power request"]
    #[inline(always)]
    #[must_use]
    pub fn set_man_pow(&mut self) -> SET_MAN_POW_W<CCR_SPEC> {
        SET_MAN_POW_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set manual or automatic invalid"]
    #[inline(always)]
    #[must_use]
    pub fn set_man_inv(&mut self) -> SET_MAN_INV_W<CCR_SPEC> {
        SET_MAN_INV_W::new(self, 4)
    }
    #[doc = "Bit 5 - Prefetch value"]
    #[inline(always)]
    #[must_use]
    pub fn set_prefetch(&mut self) -> SET_PREFETCH_W<CCR_SPEC> {
        SET_PREFETCH_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable statistics"]
    #[inline(always)]
    #[must_use]
    pub fn statistic_en(&mut self) -> STATISTIC_EN_W<CCR_SPEC> {
        STATISTIC_EN_W::new(self, 6)
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
#[doc = "Configuration and control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0x40"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
