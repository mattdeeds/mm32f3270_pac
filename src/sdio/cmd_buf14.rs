#[doc = "Register `CMD_BUF14` reader"]
pub type R = crate::R<CMD_BUF14_SPEC>;
#[doc = "Register `CMD_BUF14` writer"]
pub type W = crate::W<CMD_BUF14_SPEC>;
#[doc = "Field `DAT` reader - cmd_buf byte 14, mapped to command bit 127 to bit 120 bits"]
pub type DAT_R = crate::FieldReader;
#[doc = "Field `DAT` writer - cmd_buf byte 14, mapped to command bit 127 to bit 120 bits"]
pub type DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cmd_buf byte 14, mapped to command bit 127 to bit 120 bits"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cmd_buf byte 14, mapped to command bit 127 to bit 120 bits"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<CMD_BUF14_SPEC> {
        DAT_W::new(self, 0)
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
#[doc = "CMD buffer register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_buf14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_buf14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_BUF14_SPEC;
impl crate::RegisterSpec for CMD_BUF14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_buf14::R`](R) reader structure"]
impl crate::Readable for CMD_BUF14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_buf14::W`](W) writer structure"]
impl crate::Writable for CMD_BUF14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_BUF14 to value 0"]
impl crate::Resettable for CMD_BUF14_SPEC {
    const RESET_VALUE: u32 = 0;
}
