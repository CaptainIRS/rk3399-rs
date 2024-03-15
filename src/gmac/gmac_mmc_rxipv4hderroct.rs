#[doc = "Register `GMAC_MMC_RXIPV4HDERROCT` reader"]
pub type R = crate::R<GmacMmcRxipv4hderroctSpec>;
#[doc = "Register `GMAC_MMC_RXIPV4HDERROCT` writer"]
pub type W = crate::W<GmacMmcRxipv4hderroctSpec>;
#[doc = "Field `RXIPV4_HDRERR_OCTETS` reader - Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
pub type Rxipv4HdrerrOctetsR = crate::FieldReader<u32>;
#[doc = "Field `RXIPV4_HDRERR_OCTETS` writer - Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
pub type Rxipv4HdrerrOctetsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
    #[inline(always)]
    pub fn rxipv4_hdrerr_octets(&self) -> Rxipv4HdrerrOctetsR {
        Rxipv4HdrerrOctetsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4_hdrerr_octets(&mut self) -> Rxipv4HdrerrOctetsW<GmacMmcRxipv4hderroctSpec> {
        Rxipv4HdrerrOctetsW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET IPV4 Head Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv4hderroct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv4hderroct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxipv4hderroctSpec;
impl crate::RegisterSpec for GmacMmcRxipv4hderroctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxipv4hderroct::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxipv4hderroctSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxipv4hderroct::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxipv4hderroctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXIPV4HDERROCT to value 0"]
impl crate::Resettable for GmacMmcRxipv4hderroctSpec {
    const RESET_VALUE: u32 = 0;
}