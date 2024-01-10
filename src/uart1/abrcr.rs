#[doc = "Register `ABRCR` reader"]
pub type R = crate::R<ABRCR_SPEC>;
#[doc = "Register `ABRCR` writer"]
pub type W = crate::W<ABRCR_SPEC>;
#[doc = "Field `Abren` reader - Automatic baud rate enable"]
pub type ABREN_R = crate::BitReader;
#[doc = "Field `Abren` writer - Automatic baud rate enable"]
pub type ABREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Abr_bitcnt` reader - Automatic baud rate detection length"]
pub type ABR_BITCNT_R = crate::FieldReader;
#[doc = "Field `Abr_bitcnt` writer - Automatic baud rate detection length"]
pub type ABR_BITCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Former_edge` reader - Auto baud rate previous edge selection"]
pub type FORMER_EDGE_R = crate::BitReader;
#[doc = "Field `Former_edge` writer - Auto baud rate previous edge selection"]
pub type FORMER_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Later_edge` reader - Automatic baud rate after edge selection"]
pub type LATER_EDGE_R = crate::BitReader;
#[doc = "Field `Later_edge` writer - Automatic baud rate after edge selection"]
pub type LATER_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Automatic baud rate enable"]
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Automatic baud rate detection length"]
    #[inline(always)]
    pub fn abr_bitcnt(&self) -> ABR_BITCNT_R {
        ABR_BITCNT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Auto baud rate previous edge selection"]
    #[inline(always)]
    pub fn former_edge(&self) -> FORMER_EDGE_R {
        FORMER_EDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic baud rate after edge selection"]
    #[inline(always)]
    pub fn later_edge(&self) -> LATER_EDGE_R {
        LATER_EDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic baud rate enable"]
    #[inline(always)]
    #[must_use]
    pub fn abren(&mut self) -> ABREN_W<ABRCR_SPEC> {
        ABREN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Automatic baud rate detection length"]
    #[inline(always)]
    #[must_use]
    pub fn abr_bitcnt(&mut self) -> ABR_BITCNT_W<ABRCR_SPEC> {
        ABR_BITCNT_W::new(self, 1)
    }
    #[doc = "Bit 3 - Auto baud rate previous edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn former_edge(&mut self) -> FORMER_EDGE_W<ABRCR_SPEC> {
        FORMER_EDGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic baud rate after edge selection"]
    #[inline(always)]
    #[must_use]
    pub fn later_edge(&mut self) -> LATER_EDGE_W<ABRCR_SPEC> {
        LATER_EDGE_W::new(self, 4)
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
#[doc = "Automatic Baud Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ABRCR_SPEC;
impl crate::RegisterSpec for ABRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abrcr::R`](R) reader structure"]
impl crate::Readable for ABRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`abrcr::W`](W) writer structure"]
impl crate::Writable for ABRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABRCR to value 0"]
impl crate::Resettable for ABRCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
