/// Volume Profile - POC (Point of Control), VAH (Value Area High), VAL (Value Area Low)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeProfile {
    pub poc: f64,      // Point of Control - price with highest volume
    pub vah: f64,      // Value Area High - top of 70% volume range
    pub val: f64,      // Value Area Low - bottom of 70% volume range
    pub total_volume: f64,
}

#[derive(Debug, Clone)]
pub struct PriceVolume {
    pub price: f64,
    pub volume: f64,
}

impl VolumeProfile {
    /// Calculate volume profile from price-volume data
    pub fn calculate(data: &[PriceVolume], num_bins: usize) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        // Find price range
        let min_price = data.iter().map(|pv| pv.price).fold(f64::INFINITY, f64::min);
        let max_price = data.iter().map(|pv| pv.price).fold(f64::NEG_INFINITY, f64::max);
        
        if min_price >= max_price {
            return None;
        }

        let price_step = (max_price - min_price) / num_bins as f64;
        
        // Create volume bins
        let mut bins: Vec<f64> = vec![0.0; num_bins];
        let total_volume: f64 = data.iter().map(|pv| pv.volume).sum();
        
        // Distribute volume into bins
        for pv in data {
            let bin_idx = ((pv.price - min_price) / price_step).floor() as usize;
            let bin_idx = bin_idx.min(num_bins - 1);
            bins[bin_idx] += pv.volume;
        }
        
        // Find POC (highest volume bin)
        let (poc_idx, _) = bins.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())?;
        
        let poc = min_price + (poc_idx as f64 + 0.5) * price_step;
        
        // Calculate Value Area (70% of volume)
        let value_area_volume = total_volume * 0.70;
        let mut accumulated_volume = bins[poc_idx];
        let mut lower_idx = poc_idx;
        let mut upper_idx = poc_idx;
        
        while accumulated_volume < value_area_volume {
            let lower_vol = if lower_idx > 0 { bins[lower_idx - 1] } else { 0.0 };
            let upper_vol = if upper_idx < num_bins - 1 { bins[upper_idx + 1] } else { 0.0 };
            
            if lower_vol >= upper_vol && lower_idx > 0 {
                lower_idx -= 1;
                accumulated_volume += bins[lower_idx];
            } else if upper_idx < num_bins - 1 {
                upper_idx += 1;
                accumulated_volume += bins[upper_idx];
            } else {
                break;
            }
        }
        
        let vah = min_price + (upper_idx as f64 + 1.0) * price_step;
        let val = min_price + lower_idx as f64 * price_step;
        
        Some(VolumeProfile {
            poc,
            vah,
            val,
            total_volume,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_profile() {
        let data = vec![
            PriceVolume { price: 100.0, volume: 10.0 },
            PriceVolume { price: 101.0, volume: 20.0 },
            PriceVolume { price: 102.0, volume: 30.0 },
            PriceVolume { price: 103.0, volume: 20.0 },
            PriceVolume { price: 104.0, volume: 10.0 },
        ];
        
        let vp = VolumeProfile::calculate(&data, 10).unwrap();
        assert!(vp.poc > 101.0 && vp.poc < 103.0);
        assert!(vp.vah > vp.poc);
        assert!(vp.val < vp.poc);
    }
}
