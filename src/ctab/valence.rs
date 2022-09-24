use std::fmt::Display;

use super::{Charge, Element, Error};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Valence(u8);

impl Valence {
    pub fn compute(
        element: &Element,
        charge: &Charge,
        virtual_hydrogens: usize,
        bond_order_sum: usize,
    ) -> Result<Option<Self>, Error> {
        match element.isoelectronic(&charge) {
            Some(element) => match element.default_valences() {
                Some(valences) => {
                    let sum = virtual_hydrogens + bond_order_sum;

                    for valence in valences {
                        if &sum == valence {
                            return Ok(None);
                        }
                    }

                    Ok(Some(Self::try_from(sum)?))
                }
                None => {
                    if virtual_hydrogens == 0 {
                        Ok(None)
                    } else {
                        Ok(Some(Self::try_from(
                            virtual_hydrogens + bond_order_sum,
                        )?))
                    }
                }
            },
            None => Ok(None),
        }
    }
}

impl TryFrom<usize> for Valence {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 15 {
            Ok(Valence(value as u8))
        } else {
            Err(Error::InvalidValence)
        }
    }
}

impl From<&Valence> for u8 {
    fn from(value: &Valence) -> Self {
        value.0
    }
}

impl From<&Valence> for usize {
    fn from(value: &Valence) -> Self {
        value.0 as usize
    }
}

impl Display for Valence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            f.write_str("-1")
        } else {
            self.0.fmt(f)
        }
    }
}

#[cfg(test)]
mod compute {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn unmodeled_charge_0_hydrogens_0_bosum_0() {
        let element = Element::He;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 0;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(None)
        )
    }

    #[test]
    fn unmodeled_charge_0_hydrogens_0_bosum_1() {
        let element = Element::He;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 1;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(None)
        )
    }

    #[test]
    fn unmodeled_charge_0_hydrogens_1_bosum_0() {
        let element = Element::Li;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 1;
        let bond_order_sum = 0;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(Some(Valence(1)))
        )
    }

    #[test]
    fn unmodeled_charge_0_hydrogens_1_bosum_1() {
        let element = Element::Ca;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 1;
        let bond_order_sum = 1;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(Some(Valence(2)))
        )
    }

    #[test]
    fn modeled_charge_0_hydrogens_0_bosum_0() {
        let element = Element::C;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 0;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(Some(Valence(0)))
        )
    }

    #[test]
    fn modeled_charge_0_hydrogens_1_bosum_1() {
        let element = Element::C;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 1;
        let bond_order_sum = 1;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(Some(Valence(2)))
        )
    }

    #[test]
    fn modeled_charge_1_hydrogens_0_bosum_0() {
        let element = Element::C;
        let charge = 1.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 0;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(Some(Valence(0)))
        )
    }

    #[test]
    fn modeled_charge_1_hydrogens_0_bosum_3() {
        let element = Element::C;
        let charge = 1.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 3;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(None)
        )
    }

    #[test]
    fn modeled_charge_0_hydrogens_15_bosum_0() {
        let element = Element::C;
        let charge = 1.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 15;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Err(Error::InvalidValence)
        )
    }

    #[test]
    fn unmodeled_charge_1_hydrogens_0_bosum_3() {
        let element = Element::C;
        let charge = 2.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 2;

        assert_eq!(
            Valence::compute(
                &element,
                &charge,
                virtual_hydrogens,
                bond_order_sum
            ),
            Ok(None)
        )
    }
}
