#[doc = "Register `GRF_PCIE_PERF_RD_AXI_TOTAL_BYTE` reader"]
pub type R = crate::R<GrfPciePerfRdAxiTotalByteSpec>;
#[doc = "Register `GRF_PCIE_PERF_RD_AXI_TOTAL_BYTE` writer"]
pub type W = crate::W<GrfPciePerfRdAxiTotalByteSpec>;
#[doc = "Field `RD_AXI_TOTAL_BYTE` reader - AXI active total read bytes/ddr align read bytes"]
pub type RdAxiTotalByteR = crate::FieldReader<u32>;
#[doc = "Field `RD_AXI_TOTAL_BYTE` writer - AXI active total read bytes/ddr align read bytes"]
pub type RdAxiTotalByteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AXI active total read bytes/ddr align read bytes"]
    #[inline(always)]
    pub fn rd_axi_total_byte(&self) -> RdAxiTotalByteR {
        RdAxiTotalByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AXI active total read bytes/ddr align read bytes"]
    #[inline(always)]
    #[must_use]
    pub fn rd_axi_total_byte(&mut self) -> RdAxiTotalByteW<GrfPciePerfRdAxiTotalByteSpec> {
        RdAxiTotalByteW::new(self, 0)
    }
}
#[doc = "pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_rd_axi_total_byte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_rd_axi_total_byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfPciePerfRdAxiTotalByteSpec;
impl crate::RegisterSpec for GrfPciePerfRdAxiTotalByteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_pcie_perf_rd_axi_total_byte::R`](R) reader structure"]
impl crate::Readable for GrfPciePerfRdAxiTotalByteSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_pcie_perf_rd_axi_total_byte::W`](W) writer structure"]
impl crate::Writable for GrfPciePerfRdAxiTotalByteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_PCIE_PERF_RD_AXI_TOTAL_BYTE to value 0"]
impl crate::Resettable for GrfPciePerfRdAxiTotalByteSpec {
    const RESET_VALUE: u32 = 0;
}