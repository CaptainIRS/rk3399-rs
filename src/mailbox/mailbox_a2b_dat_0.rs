#[doc = "Register `MAILBOX_A2B_DAT_0` reader"]
pub type R = crate::R<MailboxA2bDat0Spec>;
#[doc = "Register `MAILBOX_A2B_DAT_0` writer"]
pub type W = crate::W<MailboxA2bDat0Spec>;
#[doc = "Field `DATA` reader - data of Cortex-A53/Cortex-A72 to Cortex-M0"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - data of Cortex-A53/Cortex-A72 to Cortex-M0"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data of Cortex-A53/Cortex-A72 to Cortex-M0"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data of Cortex-A53/Cortex-A72 to Cortex-M0"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MailboxA2bDat0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_dat_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_dat_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxA2bDat0Spec;
impl crate::RegisterSpec for MailboxA2bDat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_a2b_dat_0::R`](R) reader structure"]
impl crate::Readable for MailboxA2bDat0Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_a2b_dat_0::W`](W) writer structure"]
impl crate::Writable for MailboxA2bDat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_A2B_DAT_0 to value 0"]
impl crate::Resettable for MailboxA2bDat0Spec {
    const RESET_VALUE: u32 = 0;
}
