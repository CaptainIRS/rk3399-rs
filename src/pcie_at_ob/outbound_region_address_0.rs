#[doc = "Register `OUTBOUND_REGION_ADDRESS_0` reader"]
pub type R = crate::R<OutboundRegionAddress0Spec>;
#[doc = "Register `OUTBOUND_REGION_ADDRESS_0` writer"]
pub type W = crate::W<OutboundRegionAddress0Spec>;
#[doc = "Field `num_bits` reader - Number_bits \\[5:0\\]
\\[num_bits\\]
Number of bits of the addres sthat are valid"]
pub type NumBitsR = crate::FieldReader;
#[doc = "Field `num_bits` writer - Number_bits \\[5:0\\]
\\[num_bits\\]
Number of bits of the addres sthat are valid"]
pub type NumBitsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `data` reader - Address bits \\[31:8\\]
\\[data\\]
Lower 32-bits of Address Register for region N"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Address bits \\[31:8\\]
\\[data\\]
Lower 32-bits of Address Register for region N"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:5 - Number_bits \\[5:0\\]
\\[num_bits\\]
Number of bits of the addres sthat are valid"]
    #[inline(always)]
    pub fn num_bits(&self) -> NumBitsR {
        NumBitsR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Address bits \\[31:8\\]
\\[data\\]
Lower 32-bits of Address Register for region N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number_bits \\[5:0\\]
\\[num_bits\\]
Number of bits of the addres sthat are valid"]
    #[inline(always)]
    #[must_use]
    pub fn num_bits(&mut self) -> NumBitsW<OutboundRegionAddress0Spec> {
        NumBitsW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Address bits \\[31:8\\]
\\[data\\]
Lower 32-bits of Address Register for region N"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<OutboundRegionAddress0Spec> {
        DataW::new(self, 8)
    }
}
#[doc = "Outbound Region Address 0 Lower 32-bits of Address Register for region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbound_region_address_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbound_region_address_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutboundRegionAddress0Spec;
impl crate::RegisterSpec for OutboundRegionAddress0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outbound_region_address_0::R`](R) reader structure"]
impl crate::Readable for OutboundRegionAddress0Spec {}
#[doc = "`write(|w| ..)` method takes [`outbound_region_address_0::W`](W) writer structure"]
impl crate::Writable for OutboundRegionAddress0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTBOUND_REGION_ADDRESS_0 to value 0"]
impl crate::Resettable for OutboundRegionAddress0Spec {
    const RESET_VALUE: u32 = 0;
}