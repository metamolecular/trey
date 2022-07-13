use super::{Charge, Element};

pub fn valence(
    element: &Element,
    charge: &Charge,
    virtual_hydrogens: usize,
    bond_order_sum: usize,
) -> Option<usize> {
    match element.isoelectronic(&charge) {
        Some(element) => match element.default_valences() {
            Some(valences) => {
                let sum = virtual_hydrogens + bond_order_sum;

                for valence in valences {
                    if &sum == valence {
                        return None;
                    }
                }

                Some(sum)
            }
            None => {
                if virtual_hydrogens == 0 {
                    None
                } else {
                    Some(virtual_hydrogens + bond_order_sum)
                }
            }
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn unmodeled_charge_0_hydrogens_0_bosum_0() {
        let element = Element::He;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 0;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            None
        )
    }

    #[test]
    fn unmodeled_charge_0_hydrogens_0_bosum_1() {
        let element = Element::He;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 1;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            None
        )
    }

    #[test]
    fn unmodeled_charge_0_hydrogens_1_bosum_0() {
        let element = Element::Li;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 1;
        let bond_order_sum = 0;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            Some(1)
        )
    }

    #[test]
    fn unmodeled_charge_0_hydrogens_1_bosum_1() {
        let element = Element::Ca;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 1;
        let bond_order_sum = 1;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            Some(2)
        )
    }

    #[test]
    fn modeled_charge_0_hydrogens_0_bosum_0() {
        let element = Element::C;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 0;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            Some(0)
        )
    }

    #[test]
    fn modeled_charge_0_hydrogens_1_bosum_1() {
        let element = Element::C;
        let charge = 0.try_into().unwrap();
        let virtual_hydrogens = 1;
        let bond_order_sum = 1;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            Some(2)
        )
    }

    #[test]
    fn modeled_charge_1_hydrogens_0_bosum_0() {
        let element = Element::C;
        let charge = 1.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 0;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            Some(0)
        )
    }

    #[test]
    fn modeled_charge_1_hydrogens_0_bosum_3() {
        let element = Element::C;
        let charge = 1.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 3;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            None
        )
    }

    #[test]
    fn unmodeled_charge_1_hydrogens_0_bosum_3() {
        let element = Element::C;
        let charge = 2.try_into().unwrap();
        let virtual_hydrogens = 0;
        let bond_order_sum = 2;

        assert_eq!(
            valence(&element, &charge, virtual_hydrogens, bond_order_sum),
            None
        )
    }
}
