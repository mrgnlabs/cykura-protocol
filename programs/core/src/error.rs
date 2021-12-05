use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("LOK")]
    LOK,
    #[msg("Minting amount should be greater than 0")]
    ZeroMintAmount,

    // states/pool.rs

    // The lower tick must be below the upper tick
    #[msg("TLU")]
    TLU,

    // The lower tick must be greater, or equal to, the minimum tick
    #[msg("TLM")]
    TLM,

    // The upper tick must be lesser than, or equal to, the maximum tick
    #[msg("TUM")]
    TUM,

    // Mint 0, The balance of token0 in the given pool before minting must be less than,
    // or equal to, the balance after minting
    #[msg("M0")]
    M0,

    // Mint 1, The balance of token1 in the given pool before minting must be less than,
    // or equal to, the balance after minting
    #[msg("M1")]
    M1,

    // Observation state seed should be valid
    #[msg("OS")]
    OS,

    // libraries/tick_math.rs

    // second inequality must be < because the price can never reach the price at the max tick
    #[msg("R")]
    R,
    // The given tick must be less than, or equal to, the maximum tick
    #[msg("T")]
    T,

    // libraries/liquidity_math.rs

    // Liquidity Sub
    #[msg("LS")]
    LS,

    // Liquidity Add
    #[msg("LA")]
    LA,
}
