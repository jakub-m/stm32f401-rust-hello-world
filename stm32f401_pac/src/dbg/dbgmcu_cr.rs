#[doc = "Register `DBGMCU_CR` reader"]
pub type R = crate::R<DbgmcuCrSpec>;
#[doc = "Register `DBGMCU_CR` writer"]
pub type W = crate::W<DbgmcuCrSpec>;
#[doc = "Field `DBG_SLEEP` reader - DBG_SLEEP"]
pub type DbgSleepR = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - DBG_SLEEP"]
pub type DbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - DBG_STOP"]
pub type DbgStopR = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - DBG_STOP"]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` reader - DBG_STANDBY"]
pub type DbgStandbyR = crate::BitReader;
#[doc = "Field `DBG_STANDBY` writer - DBG_STANDBY"]
pub type DbgStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub type TraceIoenR = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub type TraceIoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub type TraceModeR = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub type TraceModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DbgSleepR {
        DbgSleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DbgStopR {
        DbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DbgStandbyR {
        DbgStandbyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TraceIoenR {
        TraceIoenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TraceModeR {
        TraceModeR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DbgSleepW<DbgmcuCrSpec> {
        DbgSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DbgStopW<DbgmcuCrSpec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DbgStandbyW<DbgmcuCrSpec> {
        DbgStandbyW::new(self, 2)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TraceIoenW<DbgmcuCrSpec> {
        TraceIoenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TraceModeW<DbgmcuCrSpec> {
        TraceModeW::new(self, 6)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgmcu_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuCrSpec;
impl crate::RegisterSpec for DbgmcuCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_cr::R`](R) reader structure"]
impl crate::Readable for DbgmcuCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgmcu_cr::W`](W) writer structure"]
impl crate::Writable for DbgmcuCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGMCU_CR to value 0"]
impl crate::Resettable for DbgmcuCrSpec {
    const RESET_VALUE: u32 = 0;
}
