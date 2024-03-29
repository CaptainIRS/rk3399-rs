#[doc = "Register `CSC_COEF_A1_MSB` reader"]
pub type R = crate::R<CscCoefA1MsbSpec>;
#[doc = "Register `CSC_COEF_A1_MSB` writer"]
pub type W = crate::W<CscCoefA1MsbSpec>;
#[doc = "Field `CSC_COEF_A1_MSB` reader - Color Space Converter Matrix A1 Coefficient\n\nRegister MSB"]
pub type CscCoefA1MsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_A1_MSB` writer - Color Space Converter Matrix A1 Coefficient\n\nRegister MSB"]
pub type CscCoefA1MsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A1 Coefficient\n\nRegister MSB"]
    #[inline(always)]
    pub fn csc_coef_a1_msb(&self) -> CscCoefA1MsbR {
        CscCoefA1MsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A1 Coefficient\n\nRegister MSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_a1_msb(&mut self) -> CscCoefA1MsbW<CscCoefA1MsbSpec> {
        CscCoefA1MsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix A1 Coefficient Register MSB Notes:\n\nThe coefficients used in the CSC matrix use only 15 bits for the internal computations.\n\nCoefficients are represented in 2's complementary format and stored in two registers:\n\ncsc_coef_*_lsb\\[7:0\\]: coefficient bits 7 to 0\n\ncsc_coef_*_msb\\[7\\]: spare bit\n\ncsc_coef_*_msb\\[6:0\\]: coefficient bits 14 to 8\n\nExamples for standard ITU601 and ITU709 RGB/YCC conversion CSC coefficients exist in\n\nthe Video Datapath Application Note.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a1_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a1_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefA1MsbSpec;
impl crate::RegisterSpec for CscCoefA1MsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_a1_msb::R`](R) reader structure"]
impl crate::Readable for CscCoefA1MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_a1_msb::W`](W) writer structure"]
impl crate::Writable for CscCoefA1MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_A1_MSB to value 0x20"]
impl crate::Resettable for CscCoefA1MsbSpec {
    const RESET_VALUE: u8 = 0x20;
}
