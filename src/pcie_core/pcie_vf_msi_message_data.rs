#[doc = "Register `PCIE_VF_MSI_MESSAGE_DATA` reader"]
pub type R = crate::R<PcieVfMsiMessageDataSpec>;
#[doc = "Register `PCIE_VF_MSI_MESSAGE_DATA` writer"]
pub type W = crate::W<PcieVfMsiMessageDataSpec>;
#[doc = "Field `MD` reader - Message Data \\[MD\\]\n\nMessage data to be used for this\n\nFunction. This field can also be\n\nwritten from the local management\n\nbus."]
pub type MdR = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - Message Data \\[MD\\]\n\nMessage data to be used for this\n\nFunction. This field can also be\n\nwritten from the local management\n\nbus."]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]\n\nHardwired to 0"]
pub type R2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Message Data \\[MD\\]\n\nMessage data to be used for this\n\nFunction. This field can also be\n\nwritten from the local management\n\nbus."]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R2\\]\n\nHardwired to 0"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message Data \\[MD\\]\n\nMessage data to be used for this\n\nFunction. This field can also be\n\nwritten from the local management\n\nbus."]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MdW<PcieVfMsiMessageDataSpec> {
        MdW::new(self, 0)
    }
}
#[doc = "MSI Message Data Register\n\nHardwired to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_message_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_msi_message_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfMsiMessageDataSpec;
impl crate::RegisterSpec for PcieVfMsiMessageDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_msi_message_data::R`](R) reader structure"]
impl crate::Readable for PcieVfMsiMessageDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_vf_msi_message_data::W`](W) writer structure"]
impl crate::Writable for PcieVfMsiMessageDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_VF_MSI_MESSAGE_DATA to value 0"]
impl crate::Resettable for PcieVfMsiMessageDataSpec {
    const RESET_VALUE: u32 = 0;
}
