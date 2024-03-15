#[doc = "Register `GPIO_SWPORTA_DDR` reader"]
pub type R = crate::R<GpioSwportaDdrSpec>;
#[doc = "Register `GPIO_SWPORTA_DDR` writer"]
pub type W = crate::W<GpioSwportaDdrSpec>;
#[doc = "Values written to this register independently control the direction of the corresponding data bit in Port A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioSwportaDdr {
    #[doc = "0: Output"]
    B0 = 0,
    #[doc = "1: Output"]
    B1 = 1,
}
impl From<GpioSwportaDdr> for u32 {
    #[inline(always)]
    fn from(variant: GpioSwportaDdr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioSwportaDdr {
    type Ux = u32;
}
#[doc = "Field `GPIO_SWPORTA_DDR` reader - Values written to this register independently control the direction of the corresponding data bit in Port A."]
pub type GpioSwportaDdrR = crate::FieldReader<GpioSwportaDdr>;
impl GpioSwportaDdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioSwportaDdr> {
        match self.bits {
            0 => Some(GpioSwportaDdr::B0),
            1 => Some(GpioSwportaDdr::B1),
            _ => None,
        }
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpioSwportaDdr::B0
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpioSwportaDdr::B1
    }
}
#[doc = "Field `GPIO_SWPORTA_DDR` writer - Values written to this register independently control the direction of the corresponding data bit in Port A."]
pub type GpioSwportaDdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioSwportaDdr>;
impl<'a, REG> GpioSwportaDdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Output"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSwportaDdr::B0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSwportaDdr::B1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Values written to this register independently control the direction of the corresponding data bit in Port A."]
    #[inline(always)]
    pub fn gpio_swporta_ddr(&self) -> GpioSwportaDdrR {
        GpioSwportaDdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Values written to this register independently control the direction of the corresponding data bit in Port A."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_swporta_ddr(&mut self) -> GpioSwportaDdrW<GpioSwportaDdrSpec> {
        GpioSwportaDdrW::new(self, 0)
    }
}
#[doc = "Port A data direction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_swporta_ddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_swporta_ddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioSwportaDdrSpec;
impl crate::RegisterSpec for GpioSwportaDdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_swporta_ddr::R`](R) reader structure"]
impl crate::Readable for GpioSwportaDdrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_swporta_ddr::W`](W) writer structure"]
impl crate::Writable for GpioSwportaDdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_SWPORTA_DDR to value 0"]
impl crate::Resettable for GpioSwportaDdrSpec {
    const RESET_VALUE: u32 = 0;
}