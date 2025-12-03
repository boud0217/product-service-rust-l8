use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Apple iPhone 17 Pro".to_string(),
            price: 1599.99,
            description: "Dernier iPhone avec puce A19, écran ProMotion et caméra améliorée.".to_string(),
            image: "Apple_iPhone_17_Pro.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Apple Watch Ultra 3".to_string(),
            price: 1099.99,
            description: "Montre connectée robuste pour sport extrême, GPS avancé et autonomie prolongée.".to_string(),
            image: "Apple_Watch_Ultra_3.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Apple AirPods Pro 3".to_string(),
            price: 329.99,
            description: "Écouteurs sans fil avec réduction de bruit active et audio spatial.".to_string(),
            image: "Apple_AirPods_Pro_3.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Samsung Galaxy S25 FE".to_string(),
            price: 1199.99,
            description: "Smartphone Android haut de gamme avec écran AMOLED et triple caméra.".to_string(),
            image: "Samsung_Galaxy_S25_FE.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Samsung Galaxy Tab S11".to_string(),
            price: 999.99,
            description: "Tablette avec écran 11 pouces AMOLED, stylet S-Pen et multitâche optimisé.".to_string(),
            image: "Samsung_Galaxy_Tab_S11.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Google Pixel 10".to_string(),
            price: 1099.99,
            description: "Téléphone Android avec caméra IA avancée et Android 15 natif.".to_string(),
            image: "Google_Pixel_10.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Google Pixel Watch 4".to_string(),
            price: 449.99,
            description: "Montre connectée avec suivi santé, intégration Fitbit et Google Assistant.".to_string(),
            image: "Google_Pixel_Watch_4.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Logitech MX Master 4".to_string(),
            price: 149.99,
            description: "Souris ergonomique haut de gamme pour productivité et design élégant.".to_string(),
            image: "Logitech_MX_Master_4.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Oura Ring 4 Ceramic".to_string(),
            price: 399.99,
            description: "Bague connectée pour suivi santé, sommeil et activité physique.".to_string(),
            image: "Oura_Ring_4_Ceramic.jpg".to_string()
        },
        Product {
            id: 10,
            name: "ROG Ally X".to_string(),
            price: 999.99,
            description: "Console portable gaming avec processeur AMD et écran haute fréquence.".to_string(),
            image: "ROG_Ally_X.jpg".to_string()
        }
    ]

}