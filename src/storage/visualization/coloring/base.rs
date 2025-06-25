// Basic data containers and helper functions

use palette::{LinSrgb, Mix, Srgb};

/// Helper method: Push an interpolation between two colors into the target vector.
/// first_color is pushed first
/// last_color would be pushed last but isn't!
/// Linear interpolations between the two colors are pushed in between.
fn push_interpolation_part(
    target: &mut Vec<Srgb<u8>>,
    stripe_count: usize,
    first_color: Srgb<f32>,
    last_color: Srgb<f32>,
) {
    if stripe_count > 0 {
        let first_lin: LinSrgb = first_color.into();
        let last_lin: LinSrgb = last_color.into();
        for stripe in 0..stripe_count {
            let ratio = stripe as f32 / stripe_count as f32;
            let stripe_lin = first_lin.mix(last_lin, ratio);
            target.push(stripe_lin.into());
        }
    }
}

/// Scheme of a gradient coloring, describing how colors should change through the scheme
pub struct GradientColorScheme {
    body_color: Srgb<f32>,
    anchor_colors: Vec<Srgb<f32>>,
}

impl GradientColorScheme {
    pub fn new(body_color: Srgb<f32>, anchor_colors: Vec<Srgb<f32>>) -> Self {
        GradientColorScheme {
            body_color,
            anchor_colors,
        }
    }
    /// Create an interpolation of the color scheme with the given number of stripes.
    /// The anchor colors are spread evenly over the stripe with the final entries
    /// holding the interpolation from the last to the first color.
    fn create_interpolation(&self, stripe_count: usize) -> Vec<Srgb<u8>> {
        let mut target = Vec::with_capacity(stripe_count);
        let anchor_count = self.anchor_colors.len();
        let mut anchor_stripe = Vec::with_capacity(anchor_count + 1);
        let stripe_factor = stripe_count as f32 / anchor_count as f32;
        anchor_stripe.push(0);
        for i in 1..anchor_count {
            anchor_stripe.push((stripe_factor * i as f32) as usize);
        }
        anchor_stripe.push(stripe_count);
        for i in 0..anchor_count {
            push_interpolation_part(
                &mut target,
                anchor_stripe[i + 1] - anchor_stripe[i],
                self.anchor_colors[i],
                self.anchor_colors[(i + 1) % anchor_count],
            );
        }
        target
    }
}

pub struct GradientColors {
    body_color: Srgb<u8>,
    stripes: Vec<Srgb<u8>>,
}

impl GradientColors {
    /// Return new gradient colors from the given scheme.
    pub fn new(scheme: &GradientColorScheme, stripe_count: usize) -> Self {
        GradientColors {
            body_color: scheme.body_color.into(),
            stripes: scheme.create_interpolation(stripe_count),
        }
    }

    fn rgb_to_u83(color: &Srgb<u8>) -> [u8; 3] {
        [color.red, color.green, color.blue]
    }

    pub fn iteration_to_color(&self, it: u32, assigner: fn(u32) -> u32, maxit: u32) -> [u8; 3] {
        if it == maxit {
            Self::rgb_to_u83(&self.body_color)
        } else {
            Self::rgb_to_u83(&self.stripes[assigner(it) as usize % self.stripes.len()])
        }
    }
}

// end of file
