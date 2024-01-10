#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Field `POW_ERR` reader - Power error interrupt request mask"]
pub type POW_ERR_R = crate::BitReader;
#[doc = "Field `POW_ERR` writer - Power error interrupt request mask"]
pub type POW_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAN_INV_ERR` reader - Invalid manual error interrupt request mask"]
pub type MAN_INV_ERR_R = crate::BitReader;
#[doc = "Field `MAN_INV_ERR` writer - Invalid manual error interrupt request mask"]
pub type MAN_INV_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power error interrupt request mask"]
    #[inline(always)]
    pub fn pow_err(&self) -> POW_ERR_R {
        POW_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid manual error interrupt request mask"]
    #[inline(always)]
    pub fn man_inv_err(&self) -> MAN_INV_ERR_R {
        MAN_INV_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power error interrupt request mask"]
    #[inline(always)]
    #[must_use]
    pub fn pow_err(&mut self) -> POW_ERR_W<IMR_SPEC> {
        POW_ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Invalid manual error interrupt request mask"]
    #[inline(always)]
    #[must_use]
    pub fn man_inv_err(&mut self) -> MAN_INV_ERR_W<IMR_SPEC> {
        MAN_INV_ERR_W::new(self, 1)
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
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
