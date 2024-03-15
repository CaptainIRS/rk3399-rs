#[doc = "Register `CSC_COEF_B4_LSB` reader"]
pub type R = crate::R<CscCoefB4LsbSpec>;
#[doc = "Register `CSC_COEF_B4_LSB` writer"]
pub type W = crate::W<CscCoefB4LsbSpec>;
#[doc = "Field `CSC_COEF_B4_LSB` reader - Color Space Converter Matrix B4 Coefficient Register LSB"]
pub type CscCoefB4LsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_B4_LSB` writer - Color Space Converter Matrix B4 Coefficient Register LSB"]
pub type CscCoefB4LsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix B4 Coefficient Register LSB"]
    #[inline(always)]
    pub fn csc_coef_b4_lsb(&self) -> CscCoefB4LsbR {
        CscCoefB4LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix B4 Coefficient Register LSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_b4_lsb(&mut self) -> CscCoefB4LsbW<CscCoefB4LsbSpec> {
        CscCoefB4LsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix B4 Coefficient Register LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b4_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b4_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefB4LsbSpec;
impl crate::RegisterSpec for CscCoefB4LsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_b4_lsb::R`](R) reader structure"]
impl crate::Readable for CscCoefB4LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_b4_lsb::W`](W) writer structure"]
impl crate::Writable for CscCoefB4LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_B4_LSB to value 0"]
impl crate::Resettable for CscCoefB4LsbSpec {
    const RESET_VALUE: u8 = 0;
}