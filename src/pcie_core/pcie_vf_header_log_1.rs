#[doc = "Register `PCIE_VF_HEADER_LOG_1` reader"]
pub type R = crate::R<PcieVfHeaderLog1Spec>;
#[doc = "Field `HD1` reader - Header DWORD 1 \\[HD1\\]\n\nSecond DWORD of captured TLP\n\nheader STICKY."]
pub type Hd1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header DWORD 1 \\[HD1\\]\n\nSecond DWORD of captured TLP\n\nheader STICKY."]
    #[inline(always)]
    pub fn hd1(&self) -> Hd1R {
        Hd1R::new(self.bits)
    }
}
#[doc = "Header Log Register 1\n\nSecond DWORD of captured TLP\n\nheader STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_header_log_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfHeaderLog1Spec;
impl crate::RegisterSpec for PcieVfHeaderLog1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_header_log_1::R`](R) reader structure"]
impl crate::Readable for PcieVfHeaderLog1Spec {}
#[doc = "`reset()` method sets PCIE_VF_HEADER_LOG_1 to value 0"]
impl crate::Resettable for PcieVfHeaderLog1Spec {
    const RESET_VALUE: u32 = 0;
}
