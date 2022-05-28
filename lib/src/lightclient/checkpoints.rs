use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CoinList {
    pub coin_list: Vec<CheckPoints>
}

#[derive(Deserialize, Clone)]
pub struct CheckPoints {
    pub coin_name: String,
    pub coin_type: u32,
    pub chain_name: String,
    pub check_points : Vec<CheckPoint>
}

#[derive(Deserialize, Clone)]
pub struct CheckPoint {
    pub height: u64,
    pub hash: String,
    pub sapling_tree: String
}


pub fn get_closest_checkpoint(chain_name: &str, coin_type: u32, height: u64) ->  Option<(u64, String, String)> {

    let mut checkpoints: Vec<(u64, String, String)> = Vec::new();

    let cps = match reqwest::blocking::get("https://raw.githubusercontent.com/CalDescent1/piratewallet-light-cli/p2sh/coin-checkpoint.json") {
        Ok(s) => {
            match s.json::<CoinList>() {
                Ok(j) => Some(j),
                Err(_) => None
            }
        },
        Err(_) => None
    };

    match cps {
        Some(s) => {
            for coin in 0..s.coin_list.len() {
                if s.coin_list[coin].coin_type == coin_type && s.coin_list[coin].chain_name == chain_name {
                    for points in 0..s.coin_list[coin].check_points.len() {
                        checkpoints.push((s.coin_list[coin].check_points[points].height
                                        , s.coin_list[coin].check_points[points].hash.clone()
                                        , s.coin_list[coin].check_points[points].sapling_tree.clone()))
                    }
                }
            }
        },
        None => {}
    }

    find_checkpoint(height, checkpoints)
}

fn find_checkpoint(height: u64, chkpts: Vec<(u64, String, String)>) -> Option<(u64, String, String)> {
    // Find the closest checkpoint
    let mut heights = chkpts.iter().map(|(h, _, _)| *h as u64).collect::<Vec<_>>();
    heights.sort();

    match get_first_lower_than(height, heights) {
        Some(closest_height) => {
            chkpts.iter().find(|(h, _, _)| *h ==  closest_height).map(|t| (t.0, t.1.clone(), t.2.clone()))
        },
        None    => None
    }
}

fn get_first_lower_than(height: u64, heights: Vec<u64>) -> Option<u64> {
    // If it's before the first checkpoint, return None.
    if heights.len() == 0 || height < heights[0] {
        return None;
    }

    for (i, h) in heights.iter().enumerate() {
        if height < *h {
            return Some(heights[i-1]);
        }
    }

    return Some(*heights.last().unwrap());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lower_than() {
        assert_eq!(get_first_lower_than( 9, vec![10, 30, 40]), None);
        assert_eq!(get_first_lower_than(10, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(11, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(29, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(30, vec![10, 30, 40]).unwrap(), 30);
        assert_eq!(get_first_lower_than(40, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(41, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(99, vec![10, 30, 40]).unwrap(), 40);
    }

    #[test]
    fn test_checkpoints() {
        assert_eq!(get_test_checkpoint(500000), None);
        assert_eq!(get_test_checkpoint(600000).unwrap().0, 600000);
        assert_eq!(get_test_checkpoint(625000).unwrap().0, 600000);
        assert_eq!(get_test_checkpoint(650000).unwrap().0, 650000);
        assert_eq!(get_test_checkpoint(655000).unwrap().0, 650000);

        assert_eq!(get_main_checkpoint(500000), None);
        assert_eq!(get_main_checkpoint(610000).unwrap().0, 610000);
        assert_eq!(get_main_checkpoint(625000).unwrap().0, 610000);
    }

}
