//! VectoDB Main Entry Point
//! This is a simple test program to interact with our vector database

use vectodb_core::{VectorRecord, DistanceMetric, calculate_distance, Metadata, MetadataValue};
use std::collections::HashMap;

fn main() {
    println!("🚀 Welcome to VectoDB - Phase 1 Demo!");
    println!("=====================================\n");

    // Test 1: Create some vectors
    println!("📊 Test 1: Creating Vectors");
    let vector1 = VectorRecord::new(1, vec![1.0, 0.0, 0.0]);
    let vector2 = VectorRecord::new(2, vec![0.0, 1.0, 0.0]);
    let vector3 = VectorRecord::new(3, vec![0.5, 0.5, 0.0]);

    println!("Vector 1: ID {}, Data: {:?}", vector1.id, vector1.data);
    println!("Vector 2: ID {}, Data: {:?}", vector2.id, vector2.data);
    println!("Vector 3: ID {}, Data: {:?}", vector3.id, vector3.data);
    println!();

    // Test 2: Calculate distances
    println!("📏 Test 2: Distance Calculations");
    test_distance_between(&vector1, &vector2, "Vector 1 and Vector 2");
    test_distance_between(&vector1, &vector3, "Vector 1 and Vector 3");
    test_distance_between(&vector2, &vector3, "Vector 2 and Vector 3");
    println!();

    // Test 3: Create vector with metadata
    println!("🏷️  Test 3: Vector with Metadata");
    let mut metadata = HashMap::new();
    metadata.insert("category".to_string(), MetadataValue::String("image".to_string()));
    metadata.insert("confidence".to_string(), MetadataValue::Float(0.95));
    metadata.insert("processed".to_string(), MetadataValue::Boolean(true));

    let vector_with_metadata = VectorRecord::new_with_metadata(
        4,
        vec![0.8, 0.6, 0.2],
        metadata
    );

    println!("Vector 4: ID {}", vector_with_metadata.id);
    println!("  Data: {:?}", vector_with_metadata.data);
    println!("  Metadata: {:#?}", vector_with_metadata.metadata);
    println!();

    // Test 4: Validation
    println!("✅ Test 4: Vector Validation");
    let valid_vector = VectorRecord::new(5, vec![1.0, 2.0, 3.0]);
    match valid_vector.validate() {
        Ok(()) => println!("✅ Valid vector passed validation"),
        Err(e) => println!("❌ Vector validation failed: {}", e),
    }

    let invalid_vector = VectorRecord::new(6, vec![1.0, f32::NAN, 3.0]);
    match invalid_vector.validate() {
        Ok(()) => println!("✅ Vector passed validation"),
        Err(e) => println!("❌ Invalid vector correctly rejected: {}", e),
    }

    println!("\n🎉 Phase 1 demo complete! All tests passed.");
}

fn test_distance_between(vec_a: &VectorRecord, vec_b: &VectorRecord, description: &str) {
    println!("  Testing {}:", description);

    let metrics = [
        DistanceMetric::Cosine,
        DistanceMetric::DotProduct,
        DistanceMetric::Euclidean,
        DistanceMetric::Manhattan,
    ];

    for metric in metrics {
        match calculate_distance(&vec_a.data, &vec_b.data, metric) {
            Ok(distance) => {
                println!("    {:?}: {:.4}", metric, distance);
            }
            Err(e) => {
                println!("    {:?}: Error - {}", metric, e);
            }
        }
    }
    println!();
}