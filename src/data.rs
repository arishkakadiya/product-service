use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 65-inch 4K UHD Smart TV".to_string(),
            price: 799.99,
            description: "Experience cinematic visuals with the Samsung 65-inch 4K UHD Smart TV. Featuring HDR technology and smart capabilities for seamless streaming.".to_string(),
            image: "/samsung_tv.png".to_string()
        },
        Product {
            id: 2,
            name: "Apple MacBook Pro 14-inch M2 Chip".to_string(),
            price: 1999.99,
            description: "Boost your productivity with the Apple MacBook Pro. Powered by the M2 chip, it delivers lightning-fast performance and stunning Retina display visuals.".to_string(),
            image: "/macbook_pro.png".to_string()
        },
        Product {
            id: 3,
            name: "Sony WH-1000XM5 Noise-Cancelling Headphones".to_string(),
            price: 399.99,
            description: "Immerse yourself in crystal-clear audio with the Sony WH-1000XM5. These premium headphones offer industry-leading noise cancellation and up to 30 hours of battery life.".to_string(),
            image: "/sony_headphones.png".to_string()
        },
        Product {
            id: 4,
            name: "Dyson V12 Detect Slim Cordless Vacuum".to_string(),
            price: 649.99,
            description: "Keep your home spotless with the Dyson V12 Detect Slim. This powerful cordless vacuum features laser detection for precise cleaning.".to_string(),
            image: "/dyson_vacuum.png".to_string()
        },
        Product {
            id: 5,
            name: "Bose Smart Soundbar 900".to_string(),
            price: 899.99,
            description: "Elevate your home theater setup with the Bose Smart Soundbar 900. Enjoy immersive Dolby Atmos sound and seamless voice control integration.".to_string(),
            image: "/bose_soundbar.png".to_string()
        },
        Product {
            id: 6,
            name: "Ninja Foodi 8-Quart Pressure Cooker".to_string(),
            price: 199.99,
            description: "Cook delicious meals with ease using the Ninja Foodi Pressure Cooker. With 8 cooking functions and a spacious 8-quart capacity.".to_string(),
            image: "/ninja_foodi.png".to_string()
        },
        Product {
            id: 7,
            name: "GoPro HERO12 Black Action Camera".to_string(),
            price: 499.99,
            description: "Capture stunning footage with the GoPro HERO12 Black. Featuring 5.3K video resolution, HyperSmooth stabilization, and waterproof design.".to_string(),
            image: "/gopro_hero12.png".to_string()
        },
        Product {
            id: 8,
            name: "Fitbit Charge 6 Fitness Tracker".to_string(),
            price: 129.99,
            description: "Track your fitness goals with the Fitbit Charge 6. Features include heart rate monitoring, GPS, and sleep tracking in a sleek design.".to_string(),
            image: "/fitbit_charge6.png".to_string()
        },
        Product {
            id: 9,
            name: "LG 27-inch UltraGear Gaming Monitor".to_string(),
            price: 349.99,
            description: "Enhance your gaming experience with the LG UltraGear Monitor. Boasts a 144Hz refresh rate, 1ms response time, and stunning QHD resolution.".to_string(),
            image: "/lg_gaming_monitor.png".to_string()
        },
        Product {
            id: 10,
            name: "KitchenAid Artisan Stand Mixer".to_string(),
            price: 449.99,
            description: "Mix, whip, and knead with ease using the KitchenAid Artisan Stand Mixer. Features 10 speeds and a 5-quart stainless steel bowl.".to_string(),
            image: "/kitchenaid_mixer.png".to_string()
        }
        
    ]
}