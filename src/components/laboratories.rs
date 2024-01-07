//! labratory is a room or building equipped for scientific experiments, 
//! research, or teaching, or for the manufacture of drugs or chemicals.
//!
//! These are for testing out each hay in the haystack of the atomic hierarchy
//!
//! HydrogenDropper -> Does it apply its hydronic features?
//! HeliumDropper -> Does it apply its helium features?
//! WaterTheme -> Does it apply the appropiate H20 theme?
//! OxygenTheme -> Does it apply the appropiate O2 theme?
//! Dashboard -> Does it apply the organisms, molecules & atoms?
//! ... -> ...

/// Makes sure that the dropper is correctly applied
pub mod dropper;

/// Makes sure that the provided info is correct
pub mod home;

/// Makes sure that the provided information is legit
pub mod about;

/// Makes sure that user is actually downloading a target
pub mod download;

/// Creates situations which lead to a notfound page
pub mod notfound;
