struct SingleVariantMechanics {
    a: String,
}

struct MultipleVariantMechanics {
    b: String,
}

enum Mechanics {
    SingleVariantMechanics(SingleVariantMechanics),
    MultipleVariantMechanics(MultipleVariantMechanics),
}

pub fn run_example() {
    let mechanics = vec![
        Mechanics::MultipleVariantMechanics(
            MultipleVariantMechanics {
                b: "mechanics A".to_string(),
            },
        ),
        Mechanics::SingleVariantMechanics(
            SingleVariantMechanics {
                a: "mechanics B".to_string(),
            }
        ),
    ];

    for mechanic in &mechanics {
        match mechanic {
            Mechanics::SingleVariantMechanics(it) => {
                println!("[SingleVariantMechanics]: {}", it.a);
            }
            Mechanics::MultipleVariantMechanics(it) => {
                println!("[MultipleVariantMechanics]: {}", it.b);
            }
        };
    }
}