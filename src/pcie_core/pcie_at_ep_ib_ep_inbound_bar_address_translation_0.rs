#[doc = "Register `PCIE_AT_EP_IB_EP_INBOUND_BAR_ADDRESS_TRANSLATION_0` reader"]
pub type R = crate::R<PcieAtEpIbEpInboundBarAddressTranslation0Spec>;
#[doc = "Register `PCIE_AT_EP_IB_EP_INBOUND_BAR_ADDRESS_TRANSLATION_0` writer"]
pub type W = crate::W<PcieAtEpIbEpInboundBarAddressTranslation0Spec>;
#[doc = "Field `data` reader - Address bits \\[31:0\\]
\\[data\\]\n\nBits \\[31:0\\]
of Address Register for\n\nBAR N"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Address bits \\[31:0\\]
\\[data\\]\n\nBits \\[31:0\\]
of Address Register for\n\nBAR N"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address bits \\[31:0\\]
\\[data\\]\n\nBits \\[31:0\\]
of Address Register for\n\nBAR N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address bits \\[31:0\\]
\\[data\\]\n\nBits \\[31:0\\]
of Address Register for\n\nBAR N"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<PcieAtEpIbEpInboundBarAddressTranslation0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "EP Inbound BAR Address Translation 0\n\nBits \\[31:0\\]
of Address Register for\n\nBAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ep_ib_ep_inbound_bar_address_translation_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ep_ib_ep_inbound_bar_address_translation_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieAtEpIbEpInboundBarAddressTranslation0Spec;
impl crate::RegisterSpec for PcieAtEpIbEpInboundBarAddressTranslation0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_at_ep_ib_ep_inbound_bar_address_translation_0::R`](R) reader structure"]
impl crate::Readable for PcieAtEpIbEpInboundBarAddressTranslation0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_at_ep_ib_ep_inbound_bar_address_translation_0::W`](W) writer structure"]
impl crate::Writable for PcieAtEpIbEpInboundBarAddressTranslation0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_AT_EP_IB_EP_INBOUND_BAR_ADDRESS_TRANSLATION_0 to value 0"]
impl crate::Resettable for PcieAtEpIbEpInboundBarAddressTranslation0Spec {
    const RESET_VALUE: u32 = 0;
}
