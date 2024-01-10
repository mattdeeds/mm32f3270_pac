#[doc = "Register `MIIAR` reader"]
pub type R = crate::R<MIIAR_SPEC>;
#[doc = "Register `MIIAR` writer"]
pub type W = crate::W<MIIAR_SPEC>;
#[doc = "Field `MB` reader - MII Busy"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - MII Busy"]
pub type MB_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MW` reader - MII Write"]
pub type MW_R = crate::BitReader;
#[doc = "Field `MW` writer - MII Write"]
pub type MW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - Clock Range"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - Clock Range"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MR` reader - MII Register"]
pub type MR_R = crate::FieldReader;
#[doc = "Field `MR` writer - MII Register"]
pub type MR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - PHY Address"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - PHY Address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<MIIAR_SPEC> {
        MB_W::new(self, 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MW_W<MIIAR_SPEC> {
        MW_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Clock Range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MIIAR_SPEC> {
        CR_W::new(self, 2)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<MIIAR_SPEC> {
        MR_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<MIIAR_SPEC> {
        PA_W::new(self, 11)
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
#[doc = "Ethernet MAC MII address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`miiar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miiar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIIAR_SPEC;
impl crate::RegisterSpec for MIIAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miiar::R`](R) reader structure"]
impl crate::Readable for MIIAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`miiar::W`](W) writer structure"]
impl crate::Writable for MIIAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets MIIAR to value 0"]
impl crate::Resettable for MIIAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
