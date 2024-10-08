#[doc = "Register `OAR1` reader"]
pub type R = crate::R<Oar1Spec>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<Oar1Spec>;
#[doc = "Field `ADD0` reader - Interface address"]
pub type Add0R = crate::BitReader;
#[doc = "Field `ADD0` writer - Interface address"]
pub type Add0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD7` reader - Interface address"]
pub type Add7R = crate::FieldReader;
#[doc = "Field `ADD7` writer - Interface address"]
pub type Add7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADD10` reader - Interface address"]
pub type Add10R = crate::FieldReader;
#[doc = "Field `ADD10` writer - Interface address"]
pub type Add10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDMODE` reader - Addressing mode (slave mode)"]
pub type AddmodeR = crate::BitReader;
#[doc = "Field `ADDMODE` writer - Addressing mode (slave mode)"]
pub type AddmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&self) -> Add0R {
        Add0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7(&self) -> Add7R {
        Add7R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add10(&self) -> Add10R {
        Add10R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> AddmodeR {
        AddmodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add0(&mut self) -> Add0W<Oar1Spec> {
        Add0W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add7(&mut self) -> Add7W<Oar1Spec> {
        Add7W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> Add10W<Oar1Spec> {
        Add10W::new(self, 8)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn addmode(&mut self) -> AddmodeW<Oar1Spec> {
        AddmodeW::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar1Spec;
impl crate::RegisterSpec for Oar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for Oar1Spec {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for Oar1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for Oar1Spec {
    const RESET_VALUE: u32 = 0;
}
