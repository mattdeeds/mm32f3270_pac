#[doc = "Register `FFR` reader"]
pub type R = crate::R<FFR_SPEC>;
#[doc = "Register `FFR` writer"]
pub type W = crate::W<FFR_SPEC>;
#[doc = "Field `PM` reader - Promiscuous Mode"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Promiscuous Mode"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HU` reader - Hash Unicast"]
pub type HU_R = crate::BitReader;
#[doc = "Field `HU` writer - Hash Unicast"]
pub type HU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HM` reader - Hash Multicast"]
pub type HM_R = crate::BitReader;
#[doc = "Field `HM` writer - Hash Multicast"]
pub type HM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - Destination address inverse filter-ing"]
pub type DAIF_R = crate::BitReader;
#[doc = "Field `DAIF` writer - Destination address inverse filter-ing"]
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMF` reader - Pass All Multicast"]
pub type PMF_R = crate::BitReader;
#[doc = "Field `PMF` writer - Pass All Multicast"]
pub type PMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - Broadcast Frames Disable"]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - Broadcast Frames Disable"]
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Pass Control Frames"]
pub type PCF_R = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass Control Frames"]
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - Source Address Inverse Filtering"]
pub type SAIF_R = crate::BitReader;
#[doc = "Field `SAIF` writer - Source Address Inverse Filtering"]
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source Address Filter"]
pub type SAF_R = crate::BitReader;
#[doc = "Field `SAF` writer - Source Address Filter"]
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter"]
pub type HPF_R = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or Perfect Filter"]
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTFE` reader - VLAN Filter Enable"]
pub type VTFE_R = crate::BitReader;
#[doc = "Field `VTFE` writer - VLAN Filter Enable"]
pub type VTFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNTU` reader - The MAC receiver discards all IP packets without TCP or UDP fields"]
pub type DNTU_R = crate::BitReader;
#[doc = "Field `DNTU` writer - The MAC receiver discards all IP packets without TCP or UDP fields"]
pub type DNTU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RALL` reader - Receive All"]
pub type RALL_R = crate::BitReader;
#[doc = "Field `RALL` writer - Receive All"]
pub type RALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filter-ing"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pmf(&self) -> PMF_R {
        PMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Broadcast Frames Disable"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source Address Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VLAN Filter Enable"]
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - The MAC receiver discards all IP packets without TCP or UDP fields"]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn rall(&self) -> RALL_R {
        RALL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<FFR_SPEC> {
        PM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<FFR_SPEC> {
        HU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hm(&mut self) -> HM_W<FFR_SPEC> {
        HM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Destination address inverse filter-ing"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<FFR_SPEC> {
        DAIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pmf(&mut self) -> PMF_W<FFR_SPEC> {
        PMF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Broadcast Frames Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<FFR_SPEC> {
        DBF_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<FFR_SPEC> {
        PCF_W::new(self, 6)
    }
    #[doc = "Bit 8 - Source Address Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<FFR_SPEC> {
        SAIF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source Address Filter"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<FFR_SPEC> {
        SAF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<FFR_SPEC> {
        HPF_W::new(self, 10)
    }
    #[doc = "Bit 16 - VLAN Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtfe(&mut self) -> VTFE_W<FFR_SPEC> {
        VTFE_W::new(self, 16)
    }
    #[doc = "Bit 21 - The MAC receiver discards all IP packets without TCP or UDP fields"]
    #[inline(always)]
    #[must_use]
    pub fn dntu(&mut self) -> DNTU_W<FFR_SPEC> {
        DNTU_W::new(self, 21)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    #[must_use]
    pub fn rall(&mut self) -> RALL_W<FFR_SPEC> {
        RALL_W::new(self, 31)
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
#[doc = "Ethernet MAC frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FFR_SPEC;
impl crate::RegisterSpec for FFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffr::R`](R) reader structure"]
impl crate::Readable for FFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ffr::W`](W) writer structure"]
impl crate::Writable for FFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFR to value 0"]
impl crate::Resettable for FFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
