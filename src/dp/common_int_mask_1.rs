#[doc = "Register `COMMON_INT_MASK_1` reader"]
pub type R = crate::R<CommonIntMask1Spec>;
#[doc = "Register `COMMON_INT_MASK_1` writer"]
pub type W = crate::W<CommonIntMask1Spec>;
#[doc = "Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CommonIntMask1 {
    #[doc = "0: Mask interrupt"]
    B0 = 0,
    #[doc = "1: Enable interrupt"]
    B1 = 1,
}
impl From<CommonIntMask1> for u8 {
    #[inline(always)]
    fn from(variant: CommonIntMask1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CommonIntMask1 {
    type Ux = u8;
}
#[doc = "Field `COMMON_INT_MASK_1` reader - Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 1."]
pub type CommonIntMask1R = crate::FieldReader<CommonIntMask1>;
impl CommonIntMask1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CommonIntMask1> {
        match self.bits {
            0 => Some(CommonIntMask1::B0),
            1 => Some(CommonIntMask1::B1),
            _ => None,
        }
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CommonIntMask1::B0
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CommonIntMask1::B1
    }
}
#[doc = "Field `COMMON_INT_MASK_1` writer - Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 1."]
pub type CommonIntMask1W<'a, REG> = crate::FieldWriter<'a, REG, 8, CommonIntMask1>;
impl<'a, REG> CommonIntMask1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CommonIntMask1::B0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CommonIntMask1::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 1."]
    #[inline(always)]
    pub fn common_int_mask_1(&self) -> CommonIntMask1R {
        CommonIntMask1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 1."]
    #[inline(always)]
    #[must_use]
    pub fn common_int_mask_1(&mut self) -> CommonIntMask1W<CommonIntMask1Spec> {
        CommonIntMask1W::new(self, 0)
    }
}
#[doc = "Common Interrupt Mask Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_mask_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_mask_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonIntMask1Spec;
impl crate::RegisterSpec for CommonIntMask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`common_int_mask_1::R`](R) reader structure"]
impl crate::Readable for CommonIntMask1Spec {}
#[doc = "`write(|w| ..)` method takes [`common_int_mask_1::W`](W) writer structure"]
impl crate::Writable for CommonIntMask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets COMMON_INT_MASK_1 to value 0"]
impl crate::Resettable for CommonIntMask1Spec {
    const RESET_VALUE: u32 = 0;
}
