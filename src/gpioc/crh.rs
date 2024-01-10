#[doc = "Register `CRH` reader"]
pub type R = crate::R<CRH_SPEC>;
#[doc = "Register `CRH` writer"]
pub type W = crate::W<CRH_SPEC>;
#[doc = "Field `MODE8` reader - Port 8 mode bits"]
pub type MODE8_R = crate::FieldReader;
#[doc = "Field `MODE8` writer - Port 8 mode bits"]
pub type MODE8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF8` reader - Port 8 configuration bits"]
pub type CNF8_R = crate::FieldReader;
#[doc = "Field `CNF8` writer - Port 8 configuration bits"]
pub type CNF8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE9` reader - Port 9 mode bits"]
pub type MODE9_R = crate::FieldReader;
#[doc = "Field `MODE9` writer - Port 9 mode bits"]
pub type MODE9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF9` reader - Port 9 configuration bits"]
pub type CNF9_R = crate::FieldReader;
#[doc = "Field `CNF9` writer - Port 9 configuration bits"]
pub type CNF9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE10` reader - Port 10 mode bits"]
pub type MODE10_R = crate::FieldReader;
#[doc = "Field `MODE10` writer - Port 10 mode bits"]
pub type MODE10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF10` reader - Port 10 configuration bits"]
pub type CNF10_R = crate::FieldReader;
#[doc = "Field `CNF10` writer - Port 10 configuration bits"]
pub type CNF10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE11` reader - Port 11 mode bits"]
pub type MODE11_R = crate::FieldReader;
#[doc = "Field `MODE11` writer - Port 11 mode bits"]
pub type MODE11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF11` reader - Port 11 configuration bits"]
pub type CNF11_R = crate::FieldReader;
#[doc = "Field `CNF11` writer - Port 11 configuration bits"]
pub type CNF11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE12` reader - Port 12 mode bits"]
pub type MODE12_R = crate::FieldReader;
#[doc = "Field `MODE12` writer - Port 12 mode bits"]
pub type MODE12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF12` reader - Port 12 configuration bits"]
pub type CNF12_R = crate::FieldReader;
#[doc = "Field `CNF12` writer - Port 12 configuration bits"]
pub type CNF12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE13` reader - Port 13 mode bits"]
pub type MODE13_R = crate::FieldReader;
#[doc = "Field `MODE13` writer - Port 13 mode bits"]
pub type MODE13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF13` reader - Port 13 configuration bits"]
pub type CNF13_R = crate::FieldReader;
#[doc = "Field `CNF13` writer - Port 13 configuration bits"]
pub type CNF13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE14` reader - Port 14 mode bits"]
pub type MODE14_R = crate::FieldReader;
#[doc = "Field `MODE14` writer - Port 14 mode bits"]
pub type MODE14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF14` reader - Port 14 configuration bits"]
pub type CNF14_R = crate::FieldReader;
#[doc = "Field `CNF14` writer - Port 14 configuration bits"]
pub type CNF14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE15` reader - Port 15 mode bits"]
pub type MODE15_R = crate::FieldReader;
#[doc = "Field `MODE15` writer - Port 15 mode bits"]
pub type MODE15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF15` reader - Port 15 configuration bits"]
pub type CNF15_R = crate::FieldReader;
#[doc = "Field `CNF15` writer - Port 15 configuration bits"]
pub type CNF15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port 8 mode bits"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&self) -> CNF8_R {
        CNF8_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 9 mode bits"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&self) -> CNF9_R {
        CNF9_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 10 mode bits"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&self) -> CNF10_R {
        CNF10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 11 mode bits"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&self) -> CNF11_R {
        CNF11_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 12 mode bits"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&self) -> CNF12_R {
        CNF12_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 13 mode bits"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&self) -> CNF13_R {
        CNF13_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 14 mode bits"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&self) -> CNF14_R {
        CNF14_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 15 mode bits"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&self) -> CNF15_R {
        CNF15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 8 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<CRH_SPEC> {
        MODE8_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 8 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf8(&mut self) -> CNF8_W<CRH_SPEC> {
        CNF8_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 9 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<CRH_SPEC> {
        MODE9_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 9 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf9(&mut self) -> CNF9_W<CRH_SPEC> {
        CNF9_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 10 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> MODE10_W<CRH_SPEC> {
        MODE10_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 10 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf10(&mut self) -> CNF10_W<CRH_SPEC> {
        CNF10_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 11 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> MODE11_W<CRH_SPEC> {
        MODE11_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 11 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf11(&mut self) -> CNF11_W<CRH_SPEC> {
        CNF11_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 12 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> MODE12_W<CRH_SPEC> {
        MODE12_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 12 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf12(&mut self) -> CNF12_W<CRH_SPEC> {
        CNF12_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 13 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> MODE13_W<CRH_SPEC> {
        MODE13_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 13 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf13(&mut self) -> CNF13_W<CRH_SPEC> {
        CNF13_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port 14 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> MODE14_W<CRH_SPEC> {
        MODE14_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 14 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf14(&mut self) -> CNF14_W<CRH_SPEC> {
        CNF14_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port 15 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> MODE15_W<CRH_SPEC> {
        MODE15_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port 15 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf15(&mut self) -> CNF15_W<CRH_SPEC> {
        CNF15_W::new(self, 30)
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
#[doc = "configuration high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crh::R`](R) reader structure"]
impl crate::Readable for CRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crh::W`](W) writer structure"]
impl crate::Writable for CRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRH to value 0x4444_4444"]
impl crate::Resettable for CRH_SPEC {
    const RESET_VALUE: u32 = 0x4444_4444;
}
