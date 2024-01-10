#[doc = "Register `TOKEN` reader"]
pub type R = crate::R<TOKEN_SPEC>;
#[doc = "Register `TOKEN` writer"]
pub type W = crate::W<TOKEN_SPEC>;
#[doc = "Field `TOKEN_ENDPT` reader - This 4_bit value determines the endpoint address of the token command"]
pub type TOKEN_ENDPT_R = crate::FieldReader;
#[doc = "Field `TOKEN_ENDPT` writer - This 4_bit value determines the endpoint address of the token command"]
pub type TOKEN_ENDPT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOKEN_PID` reader - This 4_bit value is the token type executed by Vsub"]
pub type TOKEN_PID_R = crate::FieldReader;
#[doc = "Field `TOKEN_PID` writer - This 4_bit value is the token type executed by Vsub"]
pub type TOKEN_PID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - This 4_bit value determines the endpoint address of the token command"]
    #[inline(always)]
    pub fn token_endpt(&self) -> TOKEN_ENDPT_R {
        TOKEN_ENDPT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This 4_bit value is the token type executed by Vsub"]
    #[inline(always)]
    pub fn token_pid(&self) -> TOKEN_PID_R {
        TOKEN_PID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This 4_bit value determines the endpoint address of the token command"]
    #[inline(always)]
    #[must_use]
    pub fn token_endpt(&mut self) -> TOKEN_ENDPT_W<TOKEN_SPEC> {
        TOKEN_ENDPT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - This 4_bit value is the token type executed by Vsub"]
    #[inline(always)]
    #[must_use]
    pub fn token_pid(&mut self) -> TOKEN_PID_W<TOKEN_SPEC> {
        TOKEN_PID_W::new(self, 4)
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
#[doc = "Token register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`token::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`token::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOKEN_SPEC;
impl crate::RegisterSpec for TOKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`token::R`](R) reader structure"]
impl crate::Readable for TOKEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`token::W`](W) writer structure"]
impl crate::Writable for TOKEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOKEN to value 0"]
impl crate::Resettable for TOKEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
