struct ClothingItem {
    name: String,
    size: ClothingSize,
}

enum ClothingSize {
    Small,
    Medium,
    Large,
}

impl std::fmt::Display for ClothingSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClothingSize::Small => write!(f, "S"),
            ClothingSize::Medium => write!(f, "M"),
            ClothingSize::Large => write!(f, "L"),
        }
    }
}

fn main() {
    let shirt = ClothingItem {
        name: String::from("T-Shirt"),
        size: ClothingSize::Medium,
    };

    let jeans = ClothingItem {
        name: String::from("Jeans"),
        size: ClothingSize::Large,
    };

    let sweater = ClothingItem {
        name: String::from("Sweater"),
        size: ClothingSize::Small,
    };
    
    println!("{} (size: {})", shirt.name, shirt.size);
    println!("{} (size: {})", jeans.name, jeans.size);
    println!("{} (size: {})", sweater.name, sweater.size);
}
