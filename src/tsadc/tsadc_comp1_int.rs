#[doc = "Register `TSADC_COMP1_INT` reader"]
pub type R = crate::R<TsadcComp1IntSpec>;
#[doc = "Register `TSADC_COMP1_INT` writer"]
pub type W = crate::W<TsadcComp1IntSpec>;
#[doc = "Field `TSADC_COMP_SRC1` reader - TSADC high temperature level. TSADC output is bigger than tsadc_comp, means the temperature is high. TSADC_INT will be valid."]
pub type TsadcCompSrc1R = crate::FieldReader<u16>;
#[doc = "Field `TSADC_COMP_SRC1` writer - TSADC high temperature level. TSADC output is bigger than tsadc_comp, means the temperature is high. TSADC_INT will be valid."]
pub type TsadcCompSrc1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - TSADC high temperature level. TSADC output is bigger than tsadc_comp, means the temperature is high. TSADC_INT will be valid."]
    #[inline(always)]
    pub fn tsadc_comp_src1(&self) -> TsadcCompSrc1R {
        TsadcCompSrc1R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TSADC high temperature level. TSADC output is bigger than tsadc_comp, means the temperature is high. TSADC_INT will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn tsadc_comp_src1(&mut self) -> TsadcCompSrc1W<TsadcComp1IntSpec> {
        TsadcCompSrc1W::new(self, 0)
    }
}
#[doc = "TSADC high temperature level for source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_comp1_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_comp1_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsadcComp1IntSpec;
impl crate::RegisterSpec for TsadcComp1IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsadc_comp1_int::R`](R) reader structure"]
impl crate::Readable for TsadcComp1IntSpec {}
#[doc = "`write(|w| ..)` method takes [`tsadc_comp1_int::W`](W) writer structure"]
impl crate::Writable for TsadcComp1IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSADC_COMP1_INT to value 0"]
impl crate::Resettable for TsadcComp1IntSpec {
    const RESET_VALUE: u32 = 0;
}