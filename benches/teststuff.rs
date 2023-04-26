pub fn main() {
    let input1: (u32, u32, u64, u16) = (
        0b1111_0000_0000_1001_1111_0000_0000_1001,
        0b1000_0000_0110_1001_1111_1111_0010_1001,
        0b1000_1111_1111_1001_1000_0000_0110_1001_1111_1111_0010_1001_1111_1111_0010_1001,
        0b0101_0111_1111
    );
    inner(input1);
}

#[inline(never)]
pub(crate) fn inner(input: (u32, u32, u64, u16)) {
    let lpi = GicRedistributorLpi {
        control: RedistributorControl(input.0),
        implementer_identification: RedistributorImplementerIdentification(input.1),
        redistributor_type: RedistributorType(input.2),
    };
    assert_eq!(lpi.control.0, input.0);
    assert_eq!(lpi.implementer_identification.0, input.1);
    assert_eq!(lpi.redistributor_type.0, input.2);
    
    assert!(lpi.control.clear_enable_supported());
    assert_eq!(lpi.implementer_identification.implementer_jep106(), 2054);
}

#[derive(Debug)]
pub struct GicRedistributorLpi {
    control: RedistributorControl,
    implementer_identification: RedistributorImplementerIdentification,
    redistributor_type: RedistributorType,
}

#[derive(Debug)]
struct RedistributorControl(u32);
impl RedistributorControl {
    const fn clear_enable_supported(&self) -> bool {
        (self.0 >> 29) & 1 != 0
    }
}

#[derive(Debug)]
struct RedistributorImplementerIdentification(u32);
impl RedistributorImplementerIdentification {
    const fn implementer_jep106(&self) -> u16 {
        (self.0 >> 20) as u16 & 0b1111_1111_1111
    }
}

#[derive(Debug)]
struct RedistributorType(u64);
