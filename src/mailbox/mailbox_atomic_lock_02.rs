#[doc = "Register `MAILBOX_ATOMIC_LOCK_02` reader"]
pub type R = crate::R<MailboxAtomicLock02Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_02` writer"]
pub type W = crate::W<MailboxAtomicLock02Spec>;
#[doc = "Field `ATOMIC_LOCK_02` reader - lock flag bit 02"]
pub type AtomicLock02R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_02` writer - lock flag bit 02"]
pub type AtomicLock02W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 02"]
    #[inline(always)]
    pub fn atomic_lock_02(&self) -> AtomicLock02R {
        AtomicLock02R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 02"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_02(&mut self) -> AtomicLock02W<MailboxAtomicLock02Spec> {
        AtomicLock02W::new(self, 0)
    }
}
#[doc = "Lock flag register 02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_02::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_02::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock02Spec;
impl crate::RegisterSpec for MailboxAtomicLock02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_02::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock02Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_02::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_02 to value 0"]
impl crate::Resettable for MailboxAtomicLock02Spec {
    const RESET_VALUE: u32 = 0;
}