/// Coin chande

fn greedy_coin_change(coins: &[i32], mut amount: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for &coin in coins.iter().rev() {
        while amount >= coin {
            amount -= coin;
            result.push(coin);
        }
    }

    result
}

fn dp_coin_change(coins: &[i32], amount: i32) -> Vec<i32> {
    let mut dp = vec![None; (amount + 1) as usize];
    dp[0] = Some(Vec::new());

    for i in 1..=amount {
        for &coin in coins {
            if i >= coin {
                if let Some(ref sub_result) = dp[(i - coin) as usize] {
                    let mut current_result = sub_result.clone();
                    current_result.push(coin);

                    if dp[i as usize].is_none()
                        || current_result.len() < dp[i as usize].as_ref().unwrap().len()
                    {
                        dp[i as usize] = Some(current_result);
                    }
                }
            }
        }
    }

    dp[amount as usize].clone().unwrap_or(Vec::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn greedy_coin_change() {
        let coins = vec![1, 3, 4];
        let amount = 6;

        let result = super::greedy_coin_change(&coins, amount);
        assert_eq!(result, [4, 1, 1]);
    }

    #[test]
    fn dp_coin_change() {
        let coins = vec![1, 3, 4];
        let amount = 6;

        let result = super::dp_coin_change(&coins, amount);
        assert_eq!(result, [3, 3]);
    }
}
