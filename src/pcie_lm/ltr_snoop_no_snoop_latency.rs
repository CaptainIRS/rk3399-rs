#[doc = "Register `LTR_SNOOP_NO_SNOOP_LATENCY` reader"]
pub type R = crate::R<LtrSnoopNoSnoopLatencySpec>;
#[doc = "Register `LTR_SNOOP_NO_SNOOP_LATENCY` writer"]
pub type W = crate::W<LtrSnoopNoSnoopLatencySpec>;
#[doc = "Field `NSLV` reader - No-Snoop Latency Value \\[NSLV\\]
The client software must program this field with the value to be sent in the No-Snoop Latency Value field of the LTR message."]
pub type NslvR = crate::FieldReader<u16>;
#[doc = "Field `NSLV` writer - No-Snoop Latency Value \\[NSLV\\]
The client software must program this field with the value to be sent in the No-Snoop Latency Value field of the LTR message."]
pub type NslvW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NSLS` reader - No-Snoop Latency Scale \\[NSLS\\]
The client software must program this field with the value to be sent in the No-Snoop Latency Scale field of the LTR message."]
pub type NslsR = crate::FieldReader;
#[doc = "Field `NSLS` writer - No-Snoop Latency Scale \\[NSLS\\]
The client software must program this field with the value to be sent in the No-Snoop Latency Scale field of the LTR message."]
pub type NslsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R12` reader - Reserved \\[R12\\]
(no description)"]
pub type R12R = crate::FieldReader;
#[doc = "Field `NSLR` reader - No-Snoop Latency Requirement \\[NSLR\\]
The client software must set this bit to 1 to set the No-Snoop Latency Requirement bit in the LTR message to be sent."]
pub type NslrR = crate::BitReader;
#[doc = "Field `NSLR` writer - No-Snoop Latency Requirement \\[NSLR\\]
The client software must set this bit to 1 to set the No-Snoop Latency Requirement bit in the LTR message to be sent."]
pub type NslrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV` reader - Snoop Latency Value \\[SLV\\]
The client software must program this field with the value to be sent in the Snoop Latency Value field of the LTR message."]
pub type SlvR = crate::FieldReader<u16>;
#[doc = "Field `SLV` writer - Snoop Latency Value \\[SLV\\]
The client software must program this field with the value to be sent in the Snoop Latency Value field of the LTR message."]
pub type SlvW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLS` reader - Snoop Latency Scale \\[SLS\\]
The client software must program this field with the value to be sent in the Snoop Latency Scale field of the LTR message."]
pub type SlsR = crate::FieldReader;
#[doc = "Field `SLS` writer - Snoop Latency Scale \\[SLS\\]
The client software must program this field with the value to be sent in the Snoop Latency Scale field of the LTR message."]
pub type SlsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R13` reader - Reserved \\[R13\\]
Reserved"]
pub type R13R = crate::FieldReader;
#[doc = "Field `SL` reader - Snoop Latency \\[SL\\]
The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent."]
pub type SlR = crate::BitReader;
#[doc = "Field `SL` writer - Snoop Latency \\[SL\\]
The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent."]
pub type SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - No-Snoop Latency Value \\[NSLV\\]
The client software must program this field with the value to be sent in the No-Snoop Latency Value field of the LTR message."]
    #[inline(always)]
    pub fn nslv(&self) -> NslvR {
        NslvR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:12 - No-Snoop Latency Scale \\[NSLS\\]
The client software must program this field with the value to be sent in the No-Snoop Latency Scale field of the LTR message."]
    #[inline(always)]
    pub fn nsls(&self) -> NslsR {
        NslsR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Reserved \\[R12\\]
(no description)"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - No-Snoop Latency Requirement \\[NSLR\\]
The client software must set this bit to 1 to set the No-Snoop Latency Requirement bit in the LTR message to be sent."]
    #[inline(always)]
    pub fn nslr(&self) -> NslrR {
        NslrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Snoop Latency Value \\[SLV\\]
The client software must program this field with the value to be sent in the Snoop Latency Value field of the LTR message."]
    #[inline(always)]
    pub fn slv(&self) -> SlvR {
        SlvR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:28 - Snoop Latency Scale \\[SLS\\]
The client software must program this field with the value to be sent in the Snoop Latency Scale field of the LTR message."]
    #[inline(always)]
    pub fn sls(&self) -> SlsR {
        SlsR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:30 - Reserved \\[R13\\]
Reserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Snoop Latency \\[SL\\]
The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent."]
    #[inline(always)]
    pub fn sl(&self) -> SlR {
        SlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - No-Snoop Latency Value \\[NSLV\\]
The client software must program this field with the value to be sent in the No-Snoop Latency Value field of the LTR message."]
    #[inline(always)]
    #[must_use]
    pub fn nslv(&mut self) -> NslvW<LtrSnoopNoSnoopLatencySpec> {
        NslvW::new(self, 0)
    }
    #[doc = "Bits 10:12 - No-Snoop Latency Scale \\[NSLS\\]
The client software must program this field with the value to be sent in the No-Snoop Latency Scale field of the LTR message."]
    #[inline(always)]
    #[must_use]
    pub fn nsls(&mut self) -> NslsW<LtrSnoopNoSnoopLatencySpec> {
        NslsW::new(self, 10)
    }
    #[doc = "Bit 15 - No-Snoop Latency Requirement \\[NSLR\\]
The client software must set this bit to 1 to set the No-Snoop Latency Requirement bit in the LTR message to be sent."]
    #[inline(always)]
    #[must_use]
    pub fn nslr(&mut self) -> NslrW<LtrSnoopNoSnoopLatencySpec> {
        NslrW::new(self, 15)
    }
    #[doc = "Bits 16:25 - Snoop Latency Value \\[SLV\\]
The client software must program this field with the value to be sent in the Snoop Latency Value field of the LTR message."]
    #[inline(always)]
    #[must_use]
    pub fn slv(&mut self) -> SlvW<LtrSnoopNoSnoopLatencySpec> {
        SlvW::new(self, 16)
    }
    #[doc = "Bits 26:28 - Snoop Latency Scale \\[SLS\\]
The client software must program this field with the value to be sent in the Snoop Latency Scale field of the LTR message."]
    #[inline(always)]
    #[must_use]
    pub fn sls(&mut self) -> SlsW<LtrSnoopNoSnoopLatencySpec> {
        SlsW::new(self, 26)
    }
    #[doc = "Bit 31 - Snoop Latency \\[SL\\]
The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent."]
    #[inline(always)]
    #[must_use]
    pub fn sl(&mut self) -> SlW<LtrSnoopNoSnoopLatencySpec> {
        SlW::new(self, 31)
    }
}
#[doc = "LTR Snoop/No-Snoop Latency Register The client software must set this bit to 1 to set the Snoop Latency Requirement bit in the LTR message to be sent.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr_snoop_no_snoop_latency::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr_snoop_no_snoop_latency::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LtrSnoopNoSnoopLatencySpec;
impl crate::RegisterSpec for LtrSnoopNoSnoopLatencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltr_snoop_no_snoop_latency::R`](R) reader structure"]
impl crate::Readable for LtrSnoopNoSnoopLatencySpec {}
#[doc = "`write(|w| ..)` method takes [`ltr_snoop_no_snoop_latency::W`](W) writer structure"]
impl crate::Writable for LtrSnoopNoSnoopLatencySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTR_SNOOP_NO_SNOOP_LATENCY to value 0"]
impl crate::Resettable for LtrSnoopNoSnoopLatencySpec {
    const RESET_VALUE: u32 = 0;
}