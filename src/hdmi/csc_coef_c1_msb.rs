#[doc = "Register `CSC_COEF_C1_MSB` reader"]
pub type R = crate::R<CscCoefC1MsbSpec>;
#[doc = "Register `CSC_COEF_C1_MSB` writer"]
pub type W = crate::W<CscCoefC1MsbSpec>;
#[doc = "Field `CSC_COEF_C1_MSB` reader - Color Space Converter Matrix C1 Coefficient\n\nRegister MSB"]
pub type CscCoefC1MsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_C1_MSB` writer - Color Space Converter Matrix C1 Coefficient\n\nRegister MSB"]
pub type CscCoefC1MsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix C1 Coefficient\n\nRegister MSB"]
    #[inline(always)]
    pub fn csc_coef_c1_msb(&self) -> CscCoefC1MsbR {
        CscCoefC1MsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix C1 Coefficient\n\nRegister MSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_c1_msb(&mut self) -> CscCoefC1MsbW<CscCoefC1MsbSpec> {
        CscCoefC1MsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix C1 Coefficient Register MSB Color Space\n\nConversion C1 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c1_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c1_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefC1MsbSpec;
impl crate::RegisterSpec for CscCoefC1MsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_c1_msb::R`](R) reader structure"]
impl crate::Readable for CscCoefC1MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_c1_msb::W`](W) writer structure"]
impl crate::Writable for CscCoefC1MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_C1_MSB to value 0"]
impl crate::Resettable for CscCoefC1MsbSpec {
    const RESET_VALUE: u8 = 0;
}
