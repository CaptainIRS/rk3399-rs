#[doc = "Register `CRU_GPLL_CON0` reader"]
pub type R = crate::R<CruGpllCon0Spec>;
#[doc = "Register `CRU_GPLL_CON0` writer"]
pub type W = crate::W<CruGpllCon0Spec>;
#[doc = "Field `FBDIV` reader - Feedback Divide Value Valid divider settings are: \\[16, 3200\\]
in integer mode \\[20, 320\\]
in fractional mode Tips: no plus one operation"]
pub type FbdivR = crate::FieldReader<u16>;
#[doc = "Field `FBDIV` writer - Feedback Divide Value Valid divider settings are: \\[16, 3200\\]
in integer mode \\[20, 320\\]
in fractional mode Tips: no plus one operation"]
pub type FbdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - Feedback Divide Value Valid divider settings are: \\[16, 3200\\]
in integer mode \\[20, 320\\]
in fractional mode Tips: no plus one operation"]
    #[inline(always)]
    pub fn fbdiv(&self) -> FbdivR {
        FbdivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Feedback Divide Value Valid divider settings are: \\[16, 3200\\]
in integer mode \\[20, 320\\]
in fractional mode Tips: no plus one operation"]
    #[inline(always)]
    #[must_use]
    pub fn fbdiv(&mut self) -> FbdivW<CruGpllCon0Spec> {
        FbdivW::new(self, 0)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruGpllCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "GPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_gpll_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_gpll_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruGpllCon0Spec;
impl crate::RegisterSpec for CruGpllCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_gpll_con0::R`](R) reader structure"]
impl crate::Readable for CruGpllCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_gpll_con0::W`](W) writer structure"]
impl crate::Writable for CruGpllCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_GPLL_CON0 to value 0xc6"]
impl crate::Resettable for CruGpllCon0Spec {
    const RESET_VALUE: u32 = 0xc6;
}